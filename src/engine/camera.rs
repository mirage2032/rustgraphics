use crate::engine::transform::Transform;

pub struct Camera {
    pub transform: Transform,
}

impl Camera {
    pub fn new(transform: Transform) -> Self {
        Self {
            transform,
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            transform: Transform::default(),
        }
    }
}