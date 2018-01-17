pub enum Action {
    Forward,
    Left,
    Right,
}

pub struct Input {
    pub actions: Vec<Action>,
    pub should_exit: bool,
    pub frame_time: f32,
}

impl Input {
    pub fn new() -> Self {
        // Just making a guess on the capacity.
        let actions = Vec::with_capacity(32);

        Input {
            actions,
            should_exit: false,
            frame_time: 0.0,
        }
    }
}
