use std::time::{Duration, Instant};

pub struct TimeDelta {
    start_time: Instant,
}

impl TimeDelta {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
        }
    }

    pub fn delta(&mut self) -> Duration {
        let now = Instant::now();
        let delta = now - self.start_time;
        self.start_time = now;
        delta
    }
}

pub trait ToFps {
    fn to_fps(&self) -> f32;
}
impl ToFps for Duration {
    fn to_fps(&self) -> f32 {
        let secs = self.as_secs_f32();
        if secs == 0.0 {
            return 0.0;
        }
        1.0 / secs
    }
}