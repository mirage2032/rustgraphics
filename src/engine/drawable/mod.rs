use glam::Mat4;
use crate::engine::shader::Shader;
use crate::PROJECTION;

pub mod mesh;
pub mod cube;

pub trait Drawable {
    fn draw(&self, modelmat: &Mat4, viewmat: &Mat4);
}

pub struct DrawObject {
    mesh: Box<dyn mesh::MeshTrait>,
    shader: Shader,
}

impl Drawable for DrawObject {
    fn draw(&self,modelmat: &Mat4, viewmat: &Mat4) {
        self.shader.use_program();
        self.shader.set_mat4("model", modelmat);
        self.shader.set_mat4("view", viewmat);
        self.shader.set_mat4("projection", &PROJECTION);
        self.mesh.draw(modelmat, viewmat);
    }
}