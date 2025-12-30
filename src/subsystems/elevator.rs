use crate::Target;
use crate::constants::{elevator, pivot, robotmap};
use frcrs::ctre::{ControlMode, Talon};

pub struct Elevator {
    left: Talon,
    right: Talon,

    target_state: Target,
}

impl Elevator {
    pub fn new() -> Self {
        let left = Talon::new(robotmap::ELEVATOR_LEFT, Some("can0".to_string()));
        let right = Talon::new(robotmap::ELEVATOR_RIGHT, Some("can0".to_string()));
        //right.set_inverted(true);
        left.zero();
        right.zero();

        Self {
            left,
            right,
            target_state: Target::Stow,
        }
    }

    pub fn set_target(&mut self, target_state: Target) {
        self.target_state = target_state;
    }

    pub fn run_to_state(&mut self) {
        let target_pos = self.target_state.get_target_elevator();

        self.left.set(ControlMode::MotionMagic, target_pos);
        self.right.follow(&self.left, true);
    }

    pub fn at_target(&self) -> bool {
        let target_pos = self.target_state.get_target_elevator();
        let error = (target_pos - self.left.get_position()).abs();

        error < elevator::ERROR_THRESHOLD
    }

    pub fn get_position(&self) -> f64 {
        self.left.get_position()
    }

    pub fn zero(&self) {
        self.left.zero();
        self.right.zero();
    }

    pub fn stop(&self) {
        self.left.stop();
        self.right.stop();
    }
}
