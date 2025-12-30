use crate::Target;
use crate::constants::{elevator, intake, robotmap};
use frcrs::ctre::{ControlMode, Talon};
use frcrs::laser_can::LaserCan;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

pub struct Intake {
    intake: Talon,
    arm: Talon,
    laser_can: LaserCan,

    target_state: Target,

    last_trigger_time: Instant,
    debounce_duration: Duration,
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
            target_state: Target::Stow,
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

    pub fn intake(&mut self) {
        while !self.is_debounced() {
            self.set_intake_speed(intake::INTAKE_SPEED);
            false
        }
        true
    }

    pub fn outtake(&mut self) {
        self.intake.set(ControlMode::Percent, intake::OUTTAKE_SPEED);
    }

    pub fn set_intake_speed(&self, speed: f64) {
        self.intake.set(ControlMode::Percent, speed);
    }

    pub fn set_target(&mut self, target_state: Target) {
        self.target_state = target_state;
    }

    pub fn run_to_state(&mut self) {
        let target_rot = self.target_state.get_target_arm() / 360.0 * intake::GEAR_RATIO;

        self.arm.set(ControlMode::MotionMagic, target_rot);
    }

    pub fn at_target(&self) -> bool {
        let target_pos = self.target_state.get_target_arm();
        let error = (target_pos - self.arm.get_position()).abs();

        error < intake::ERROR_THRESHOLD
    }

    pub fn zero(&self) {
        self.arm.zero();
    }

    pub fn stop(&self) {
        self.intake.stop();
        self.arm.stop();
    }
}

