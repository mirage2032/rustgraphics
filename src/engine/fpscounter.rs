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
        return delta;
    }
}

