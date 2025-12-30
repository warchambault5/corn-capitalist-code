use crate::constants::{elevator, intake, pivot};
use crate::subsystems::{Elevator, Intake, Pivot};
use frcrs::input::Joystick;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

pub mod constants;
mod subsystems;

#[derive(Clone)]
pub struct Controllers {
    pub left_drive: Joystick,
    pub right_drive: Joystick,
    pub operator: Joystick,
}
#[derive(Clone)]
pub struct Ferris {
    pub controllers: Controllers,

    pub pivot: Rc<RefCell<Pivot>>,
    pub elevator: Rc<RefCell<Elevator>>,
    pub intake: Rc<RefCell<Intake>>,

    pub dt: Duration,
}

impl Default for Ferris {
    fn default() -> Self {
        Self::new()
    }
}

impl Ferris {
    pub fn new() -> Self {
        Ferris {
            controllers: Controllers {
                left_drive: Joystick::new(constants::joystick_map::LEFT_DRIVE),
                right_drive: Joystick::new(constants::joystick_map::RIGHT_DRIVE),
                operator: Joystick::new(constants::joystick_map::OPERATOR),
            },
            pivot: Rc::new(RefCell::new(Pivot::new())),
            elevator: Rc::new(RefCell::new(Elevator::new())),
            intake: Rc::new(RefCell::new(Intake::new())),

            dt: Duration::from_millis(0),
        }
    }

    pub fn stop(&self) {
        if let Ok(pivot) = self.pivot.try_borrow() {
            pivot.stop();
        }

        if let Ok(elevator) = self.elevator.try_borrow() {
            elevator.stop();
        }

        if let Ok(intake) = self.intake.try_borrow() {
            intake.stop();
        }
    }
}

#[derive(Clone)]
pub enum Target {
    Low,
    Mid,
    High,
    Intake,
    Stow,
}

impl Target {
    pub fn get_target_pivot(&self) -> f64 {
        match self {
            Target::Stow => pivot::STOW,
            Target::Low => pivot::SCORE,
            Target::Mid => pivot::SCORE,
            Target::High => pivot::SCORE,
            Target::Intake => pivot::INTAKE,
        }
    }

    pub fn get_target_arm(&self) -> f64 {
        match self {
            Target::Stow => intake::ARM_STOW,
            Target::Low => intake::ARM_SCORE,
            Target::Mid => intake::ARM_SCORE,
            Target::High => intake::ARM_SCORE,
            Target::Intake => intake::ARM_INTAKE,
        }
    }

    pub fn get_target_elevator(&self) -> f64 {
        match self {
            Target::Stow => elevator::STOW,
            Target::Low => elevator::LOW,
            Target::Mid => elevator::MID,
            Target::High => elevator::HIGH,
            Target::Intake => elevator::GROUND,
        }
    }
}

pub fn set_target(intake: &mut Intake, pivot: &mut Pivot, elevator: &mut Elevator, target_state: Target) {
    intake.set_target(target_state.clone());
    pivot.set_target(target_state.clone());
    elevator.set_target(target_state.clone());
}

pub fn run_to_state(intake: &mut Intake, pivot: &mut Pivot, elevator: &mut Elevator) {
    intake.run_to_state();
    pivot.run_to_state();
    elevator.run_to_state();
}
pub fn score(intake: &mut Intake, pivot: &mut Pivot, elevator: &mut Elevator, target_state: Target) {
    set_target(intake, pivot, elevator, target_state.clone());
    run_to_state(intake, pivot, elevator);

    if intake.at_target() && pivot.at_target() && elevator.at_target() {
        println!("at target");
        intake.set_intake_speed(intake::OUTTAKE_SPEED);
    }
}

pub fn intake(intake: &mut Intake, pivot: &mut Pivot, elevator: &mut Elevator) {
    set_target(intake, pivot, elevator, Target::Intake);
    run_to_state(intake, pivot, elevator);
    if intake.at_target() && pivot.at_target() && elevator.at_target() {
        println!("at target");
        intake.intake();
        //very wise comment
    }
}
