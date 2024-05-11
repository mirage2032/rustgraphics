use std::sync::Arc;

use glam::Mat4;

use crate::engine::config::STATIC_DATA;
use crate::engine::shader::Shader;

pub mod base;
pub mod mesh;

pub trait Drawable: Send {
    fn draw(&self, modelmat: &Mat4, viewmat: &Mat4);
}

pub struct DrawObject {
    mesh: Arc<Box<dyn mesh::Mesh>>,
    shader: Arc<Shader>,
}

impl Drawable for DrawObject {
    fn draw(&self, modelmat: &Mat4, viewmat: &Mat4) {
        self.shader.use_program();
        self.shader.set_mat4("model", modelmat);
        self.shader.set_mat4("view", viewmat);
        let projection = {
            let data = *STATIC_DATA
                .read()
                .expect("Failed to read config")
                .projection();
            data
        };
        self.shader.set_mat4("projection", &projection);
        self.mesh.draw();
    }
}
