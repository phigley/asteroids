#[derive(Default)]
pub struct Actions {
    pub accel_forward: bool,
    pub accel_right: bool,
    pub accel_left: bool,

    pub turn_right: bool,
    pub turn_left: bool,
}

#[derive(Default)]
pub struct Input {
    pub actions: Actions,
    pub should_exit: bool,
    pub frame_time: f32,
}
