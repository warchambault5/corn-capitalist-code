use crate::constants::{intake, robotmap};
use frcrs::ctre::{ControlMode, Talon};
use frcrs::laser_can::LaserCan;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

pub struct Intake {
    intake: Talon,
    arm: Talon,
    laser_can: LaserCan,

    arm_target: ArmState,

    last_trigger_time: Instant,
    debounce_duration: Duration,
}

pub enum ArmState {
    Intake,
    Score,
    Stow,
}

impl ArmState {
    pub fn get_target_deg(&self) -> f64 {
        match self {
            ArmState::Intake => intake::ARM_INTAKE,
            ArmState::Score => intake::ARM_SCORE,
            ArmState::Stow => intake::ARM_STOW,
        }
    }
}

impl Intake {
    pub fn new() -> Self {
        let arm = Talon::new(robotmap::ARM, Some("can0".to_string()));
        let intake = Talon::new(robotmap::INTAKE, Some("can0".to_string()));
        let laser_can = LaserCan::new(robotmap::LASER_CAN);
        arm.zero();

        Self {
            intake,
            arm,
            laser_can,
            arm_target: ArmState::Stow,
            last_trigger_time: Instant::now(),
            debounce_duration: Duration::from_millis(intake::DEBOUNCE_DURATION as u64),
        }
    }

    pub fn get_dist(&self) -> i32 {
        self.laser_can.get_measurement()
    }

    pub fn is_tripped(&self) -> bool {
        self.get_dist() < intake::INTAKE_TRIP_DISTANCE && self.get_dist() != -1
    }

    pub fn is_debounced(&mut self) -> bool {
        if self.is_tripped() && self.last_trigger_time.elapsed() >= self.debounce_duration {
            self.last_trigger_time = Instant::now();
            return true;
        }
        false
    }

    pub fn intake(intake: Rc<RefCell<Intake>>) {
        if let Ok(mut intake) = intake.try_borrow_mut() {
            while !intake.is_debounced() {
                intake.set_intake_speed(intake::INTAKE_SPEED);
            }
        }
    }

    pub fn set_intake_speed(&self, speed: f64) {
        self.intake.set(ControlMode::Percent, speed);
    }

    pub fn set_arm_state(&mut self, arm_state: ArmState) {
        self.arm_target = arm_state;
    }

    pub fn run_to_target_state(&self) {
        let target_rot = self.arm_target.get_target_deg() / 360.0 * intake::GEAR_RATIO;

        self.arm.set(ControlMode::MotionMagic, target_rot);
    }
    pub fn stop(&self) {
        self.intake.stop();
        self.arm.stop();
    }
}
