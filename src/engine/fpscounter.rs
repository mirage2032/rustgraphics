use std::time::{Duration, Instant};

pub struct FpsCounter {
    frame_count: u32,
    start_time: Instant,
    fps: f32,
}

impl FpsCounter {
    pub fn new() -> Self {
        Self {
            frame_count: 0,
            start_time: Instant::now(),
            fps: 0.0,
        }
    }

    pub fn update(&mut self) {
        self.frame_count += 1;
        let elapsed = self.start_time.elapsed();
        if elapsed >= Duration::from_secs(1) {
            self.fps = self.frame_count as f32 / elapsed.as_secs_f32();
            self.frame_count = 0;
            self.start_time = Instant::now();
        }
    }

    pub fn fps(&self) -> f32 {
        self.fps
    }
}

