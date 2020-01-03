use std;
use std::time::{Duration, Instant};

pub struct FrameTimer {
    // Time that the simulation was started, used to calculate total elapsed time.
    start_time: Instant,
    // Time of the previous frame, used to calculate frame time.
    frame_time: Instant,

    // Duration of the previous frame in seconds.
    frame_delta: f32,
}

impl FrameTimer {
    pub fn new() -> Self {
        let start_time = Instant::now();
        FrameTimer {
            start_time,
            frame_time: start_time,
            frame_delta: 0.0,
        }
    }

    pub fn update(&mut self, minimum_ms: u32, max_frame_delta: f32) -> f32 {
        // Sleep if less time than the minimum requested has elapsed.
        let minimum_duration = Duration::from_millis(u64::from(minimum_ms));
        let presleep_frame_duration = self.frame_time.elapsed();

        if let Some(sleep_duration) = minimum_duration.checked_sub(presleep_frame_duration) {
            std::thread::sleep(sleep_duration);
        };

        // Record the frame delta.
        self.frame_delta = f32::min(self.frame_time.elapsed().as_secs_f32(), max_frame_delta);

        // Start a new frame time.
        self.frame_time = Instant::now();

        self.frame_delta
    }

    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    pub fn frame_delta(&self) -> f32 {
        self.frame_delta
    }
}

impl Default for FrameTimer {
    fn default() -> Self {
        Self::new()
    }
}
