use std;
use time::{Duration, PreciseTime};

pub struct FrameTimer
{
    start_time: PreciseTime,
    frame_time: PreciseTime,
    frame_delta: f32,
}

impl FrameTimer
{
    pub fn new() -> Self {
        let start_time = PreciseTime::now();
        FrameTimer {
            start_time,
            frame_time: start_time,
            frame_delta: 0.0,
        }
    }

    pub fn update(&mut self, minimum_ms: u32, max_frame_delta: f32) -> f32 {
        let presleep_time = PreciseTime::now();
        

        let sleep_duration = Duration::milliseconds(minimum_ms as i64) - self.frame_time.to(presleep_time);
        let new_time = if let Ok(sleep_duration_std) = sleep_duration.to_std() {
            std::thread::sleep(sleep_duration_std);
            PreciseTime::now()
        } else {
            presleep_time
        };
        
        if let Some(frame_time_us) = self.frame_time.to(new_time).num_microseconds() {
            if frame_time_us > 0 {
                self.frame_delta = (frame_time_us as f32)*1e-6;
                self.frame_time = new_time;

                if self.frame_delta > max_frame_delta {
                    self.frame_delta = max_frame_delta;
                }
            }
        }

        self.frame_delta
    }

    pub fn elapsed(&self) -> Duration {
        self.start_time.to(self.frame_time)
    }

    pub fn frame_delta(&self) -> f32 {
        self.frame_delta
    }
}
