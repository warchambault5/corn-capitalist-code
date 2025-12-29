use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use frcrs::input::Joystick;
use crate::subsystems::{Elevator, Intake, Pivot};

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
