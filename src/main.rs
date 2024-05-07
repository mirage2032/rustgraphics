extern crate glfw;

use glam::{self,Mat4};
use crate::engine::Engine;

mod engine;
fn main() {
    let mut engine = Engine::new();
    engine.run();
}