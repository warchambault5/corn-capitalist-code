use crate::Target;
use crate::constants::{pivot, robotmap};
use frcrs::ctre::{ControlMode, Talon};

pub struct Pivot {
    left: Talon,
    right: Talon,

    target_state: Target,
}

impl Pivot {
    pub fn new() -> Self {
        let left = Talon::new(robotmap::PIVOT_LEFT, Some("can0".to_string()));
        let right = Talon::new(robotmap::PIVOT_RIGHT, Some("can0".to_string()));

        //right.set_inverted(true);

        Self {
            left,
            right,
            target_state: Target::Stow,
        }
    }

    pub fn get_angle(&self) -> f64 {
        let motor_rotations = self.left.get_position();
        let arm_rotations = motor_rotations / pivot::GEAR_RATIO;
        arm_rotations * 360.0
    }

    pub fn set_target(&mut self, target_state: Target) {
        self.target_state = target_state;
    }

    // pub fn run_to_state(&mut self, dt: f64) {
    //     let error = self.target_angle_deg - self.commanded_angle_deg;
    //     let max_step = pivot::MAX_DEG_PER_SEC * dt;
    //
    //     let step = error.clamp(-max_step, max_step);
    //     self.commanded_angle_deg += step;
    //
    //     let motor_rotations = self.commanded_angle_deg / 360.0 * pivot::GEAR_RATIO;
    //
    //     self.left.set(ControlMode::Position, motor_rotations);
    //     self.right.set(ControlMode::Position, motor_rotations);
    // }

    pub fn run_to_state(&mut self) {
        let target_rot = self.target_state.get_target_pivot() / 360.0 * pivot::GEAR_RATIO;

        self.left.set(ControlMode::MotionMagic, target_rot);
        //self.right.set(ControlMode::MotionMagic, target_rot);
        self.right.follow(&self.left, true);
    }

    pub fn at_target(&self) -> bool {
        let target_rot = self.target_state.get_target_pivot() / 360.0 * pivot::GEAR_RATIO;
        let error = (target_rot - self.left.get_position()).abs();

        error < pivot::ERROR_THRESHOLD
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
