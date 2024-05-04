extern crate glfw;

use glam::{self,Mat4};
use crate::engine::Engine;

mod engine;

lazy_static::lazy_static! {
    static ref WIDTH: u32 = 800;
    static ref HEIGHT: u32 = 600;
    static ref PROJECTION: Mat4 = {
    let fov: f32 = 70.0; // Field of view in degrees
    let aspect_ratio: f32 = *WIDTH as f32 / *HEIGHT as f32; // Aspect ratio of the window
    let near_clip: f32 = 0.1; // Near clipping plane
    let far_clip: f32 = 100.0; // Far clipping plane
    // Create the perspective projection matrix
    Mat4::perspective_rh(fov.to_radians(), aspect_ratio, near_clip, far_clip)
    };
}
fn main() {
    let mut engine = Engine::new();
    engine.run();
}