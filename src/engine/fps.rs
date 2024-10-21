use std::collections::VecDeque;

pub struct SmoothFps{
    last_fps: VecDeque<f32>,
    max_len: usize,
}

impl SmoothFps{
    pub fn new(max_len:usize) -> Self{
        Self{
            last_fps: VecDeque::new(),
            max_len
        }
    }

    pub fn push(&mut self, fps: f32) {
        self.last_fps.push_front(fps);
        if self.last_fps.len() > self.max_len {
            self.last_fps.pop_back();
        }
    }
    
    pub fn average(&self) -> f32 {
        let sum: f32 = self.last_fps.iter().sum();
        sum / self.last_fps.len() as f32
    }
}