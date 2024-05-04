use crate::engine::drawable::transform::Transform;
use crate::engine::shader::Shader;

pub mod mesh;
mod cube;
mod transform;

pub trait Drawable {
    fn draw(&self) {}
}

pub struct DrawObject {
    mesh: Box<dyn mesh::MeshTrait>,
    shader: Shader,
    transform: Transform,
}

impl DrawObject {
    pub fn draw(&self) {
        self.shader.use_program();
        self.mesh.draw();
    }
}