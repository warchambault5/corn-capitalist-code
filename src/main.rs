#![warn(non_snake_case)]

use cc::Ferris;
use frcrs::input::{RobotMode, RobotState};
use frcrs::networktables::{NetworkTable, SmartDashboard};
use frcrs::telemetry::Telemetry;
use frcrs::{Robot, init_hal, observe_user_program_starting, refresh_data};
use std::cell::RefCell;
use std::ops::Deref;
use std::process::exit;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::SystemTime;
use tokio::task;
use tokio::task::{AbortHandle, spawn_local};
use tokio::time::sleep;
use tokio::time::{Duration, Instant};

fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let local = task::LocalSet::new();

    let mut ferris = Rc::new(RefCell::new(Ferris::new()));
    let target_state = Target::Stow;
    // ferris.start_competition(runtime, local);

    runtime.block_on(local.run_until(async {
        if !init_hal() {
            panic!("Failed to initialize HAL");
        }

        observe_user_program_starting();

        Telemetry::init(5807);

        NetworkTable::init();

        //Telemetry::put_selector("auto chooser", Auto::names()).await;

        // SmartDashboard::start_camera_server();

        let mut last_loop = Instant::now();

        let mut auto: Option<AbortHandle> = None;

        // Watchdog setup
        let last_loop_time = Arc::new(AtomicU64::new(0));
        let watchdog_last_loop = Arc::clone(&last_loop_time);
        let watchdog_ferris = ferris.clone();

        // Spawn watchdog task
        spawn_local(async move {
            loop {
                sleep(Duration::from_millis(20)).await;
                let last = watchdog_last_loop.load(Ordering::Relaxed);
                let now = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64;

                if last != 0 && now - last > 150 {
                    println!("Loop Overrun: {}ms", now - last);
                    if let Ok(ferris) = watchdog_ferris.try_borrow_mut() {
                        ferris.stop();
                    } else {
                        println!("FAILED TO GET FERRIS TO STOP");
                        // leave commented for comp robot would like to implement abort at some point
                        // exit(1);
                    }
                    println!("Watchdog triggered: Motors stopped");
                }
            }
        });

        loop {
            refresh_data();

            let state = RobotState::get();
            let dt = last_loop.elapsed();

            if !state.enabled() {
                // if let Some(handle) = auto.take() {
                //     println!("Aborted");
                //     handle.abort();
                // }

                if let Ok(f) = ferris.try_borrow() {
                    f.stop();
                } else {
                    println!("Didnt borrow ferris");
                }
            }

            if state.enabled() && state.teleop() {
                if let Ok(mut robot) = ferris.try_borrow_mut() {
                    robot.dt = dt;
                    teleop(&mut robot).await;
                }
            }

            Telemetry::put_number("Loop Rate", 1. / dt.as_secs_f64()).await;

            let now_millis = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
            last_loop_time.store(now_millis, Ordering::Relaxed);

            let elapsed = dt.as_secs_f64();
            let left = (1. / 250. - elapsed).max(0.);
            sleep(Duration::from_secs_f64(left)).await;
            last_loop = Instant::now();
        }
    }));
}

// https://github.com/Team-2502/frcrs/blob/master/src/input/gamepad.rs
// LeftStickX: drive
// LeftStickY: drive
// RightStickX: rotate
// RightStickY: n/a
// LeftTrigger: intake  x
// RightTrigger: outtake  x
// A: n/a
// B: low  x
// X: mid  x
// Y: high  x
// DPadUp:
// DPadDown:
// DPadLeft:
// DPadRight:
// LeftBumper: set heading (would include some vison logic to see which we are scoring on and snap to heading accordingly)
// RightBumper: set heading
// Start: zero at stow
// Back:
// LeftStickButton:
// RightStickButton:
// may or may not implement these all
// TODO: add all of drivetrain :)
pub fn teleop(robot: &mut Ferris) {
    if let Ok(mut drivetrain) = robot.drivetrain.try_borrow_mut() {

    }

    if let Ok(mut intake) = robot.intake.try_borrow_mut() {
        if let Ok(mut pivot) = robot.pivot.try_borrow_mut() {
            if let Ok(mut elevator) = robot.elevator.try_borrow_mut() {
                //updates here
                //drivetrain logic here

                //set points
                if gamepad.button(2) {
                    set_target(&mut intake, &mut pivot, &mut elevator, Target::Low);
                    run_to_state(&mut intake, &mut pivot, &mut elevator);
                    if at_target(intake, pivot, elevator) {
                        robot.state = Target::Low;
                    }
                }
                else if gamepad.button(3) {
                    set_target(&mut intake, &mut pivot, &mut elevator, Target::Mid);
                    run_to_state(&mut intake, &mut pivot, &mut elevator);
                    if at_target(intake, pivot, elevator) {
                        robot.state = Target::Mid;
                    }
                }
                else if gamepad.button(4) {
                    set_target(&mut intake, &mut pivot, &mut elevator, Target::High);
                    run_to_state(&mut intake, &mut pivot, &mut elevator);
                    if at_target(intake, pivot, elevator) {
                        robot.state = Target::High;
                    }
                }

                //intake / outtake
                if gamepad.left_trigger() > 0.5 {
                    if robot.state == Target::Intake && at_target(intake, pivot, elevator) {
                        let has_bale = intake.intake();
                        if has_bale == true {
                            gamepad.left_rumble(1.);
                        } else {
                            gamepad.left_rumble(0.);
                        }
                    }
                    else {
                        set_target(intake, pivot, elevator, Target::Intake);
                        run_to_state(intake, pivot, elevator);
                        if at_target(intake, pivot, elevator) {
                            robot.state = Target::Intake;
                        }
                    }
                } else {
                    gamepad.left_rumble(0.);
                }
                if gamepad.right_trigger() > 0.5 {
                    if robot.state != Target::Stow && robot.state != Target::Intake && at_target(intake, pivot, elevator) {
                        intake.outtake();
                    }
                }
            }
        }
    }
}
