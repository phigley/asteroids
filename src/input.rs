pub struct Actions {
    pub accel_forward: bool,
    pub accel_right: bool,
    pub accel_left: bool,

    pub turn_right: bool,
    pub turn_left: bool,
}

impl Actions {
    pub fn new() -> Self {
        Actions {
            accel_forward: false,
            accel_right: false,
            accel_left: false,

            turn_right: false,
            turn_left: false,
        }
    }
}

pub struct Input {
    pub actions: Actions,
    pub should_exit: bool,
    pub frame_time: f32,
}

impl Input {
    pub fn new() -> Self {
        Input {
            actions: Actions::new(),
            should_exit: false,
            frame_time: 0.0,
        }
    }
}
