use crate::constants::{elevator, pivot, robotmap};
use frcrs::ctre::{ControlMode, Talon};

pub struct Elevator {
    left: Talon,
    right: Talon,

    target: ElevatorState,
}
#[derive(Clone)]
pub enum ElevatorState {
    Bottom,
    Ground,
    Mid,
    High,
}

impl ElevatorState {
    pub fn get_position(&self) -> f64 {
        match self {
            ElevatorState::Bottom => elevator::BOTTOM,
            ElevatorState::Ground => elevator::GROUND,
            ElevatorState::Mid => elevator::MID,
            ElevatorState::High => elevator::HIGH,
        }
    }
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
            target: ElevatorState::Bottom,
        }
    }

    pub fn set_state(&mut self, state: ElevatorState) {
        self.target = state;
    }

    pub fn run_to_state(&mut self) {
        let target_pos = self.target.get_position();

        self.left.set(ControlMode::MotionMagic, target_pos);
        self.right.follow(&self.left, true);
    }

    pub fn at_target(&self) -> bool {
        let target_pos = self.target.get_position();
        let error = (target_pos - self.left.get_position()).abs();

        error < elevator::ERROR_THRESHOLD
    }

    pub fn get_position(&self) -> f64 {
        self.left.get_position()
    }

    pub fn get_target(&self) -> ElevatorState {
        self.target.clone()
    }

    pub fn stop(&mut self) {
        self.left.stop();
        self.right.stop();
    }
}
