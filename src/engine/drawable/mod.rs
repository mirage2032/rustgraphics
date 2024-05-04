use nalgebra_glm::TMat4;

use crate::engine::shader::Shader;
use crate::PROJECTION;

pub mod mesh;
pub mod cube;

pub trait Drawable {
    fn draw(&self, modelmat: &TMat4<f32>, viewmat: &TMat4<f32>);
}

pub struct DrawObject {
    mesh: Box<dyn mesh::MeshTrait>,
    shader: Shader,
}

impl Drawable for DrawObject {
    fn draw(&self,modelmat: &TMat4<f32>, viewmat: &TMat4<f32>) {
        self.shader.use_program();
        self.shader.set_mat4("model", modelmat);
        self.shader.set_mat4("view", viewmat);
        self.shader.set_mat4("projection", &PROJECTION);
        self.mesh.draw(modelmat, viewmat);
    }
}