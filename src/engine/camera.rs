use crate::engine::transform::Transform;

pub struct Camera {
    transform: Transform,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            transform: Transform::default(),
        }
    }
}