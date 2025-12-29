pub mod robotmap {
    pub const PIVOT_LEFT: i32 = 0;
    pub const PIVOT_RIGHT: i32 = 1;

    pub const ELEVATOR_LEFT: i32 = 2;
    pub const ELEVATOR_RIGHT: i32 = 3;

    pub const ARM: i32 = 4;
    pub const INTAKE: i32 = 5;
    pub const LASER_CAN: i32 = 6;
}

pub mod joystick_map {
    pub const RIGHT_DRIVE: i32 = 0;
    pub const LEFT_DRIVE: i32 = 1;
    pub const OPERATOR: i32 = 2;
}

pub mod pivot {
    pub const STOW: f64 = 90.;
    pub const SCORE: f64 = 45.;
    pub const INTAKE: f64 = 0.;

    pub const GEAR_RATIO: f64 = 0.;
    //pub const MAX_DEG_PER_SEC: f32 = 0.;
    pub const ERROR_THRESHOLD: f64 = 0.5;
}

pub mod elevator {
    pub const STOW: f64 = 0.;
    pub const LOW: f64 = 0.;
    pub const GROUND: f64 = 0.;
    pub const MID: f64 = 0.;
    pub const HIGH: f64 = 0.;

    pub const ERROR_THRESHOLD: f64 = 0.5;
}

pub mod intake {
    pub const ARM_INTAKE: f64 = 1.;
    pub const ARM_STOW: f64 = 2.;
    pub const ARM_SCORE: f64 = 3.;
    pub const GEAR_RATIO: f64 = 0.;

    pub const INTAKE_SPEED: f64 = 0.;
    pub const OUTTAKE_SPEED: f64 = 0.;
    pub const INTAKE_TRIP_DISTANCE: i32 = 10; //mm
    pub const DEBOUNCE_DURATION: i32 = 200; //milli 
    pub const ERROR_THRESHOLD: f64 = 0.5;
}
