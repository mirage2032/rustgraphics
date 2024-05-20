use std::sync::{Arc, Mutex};

use glam::Mat4;

use shader::Shader;

use crate::engine::config::STATIC_DATA;
use crate::engine::drawable::mesh::unbind;

pub mod base;
pub mod importer;
pub mod mesh;
pub mod shader;

pub trait Draw: Send {
    fn draw(&self, modelmat: &Mat4, viewmat: &Mat4);
}

pub struct DrawData {
    mesh: Arc<Mutex<dyn mesh::Mesh>>,
    shader: Arc<Shader>,
    material: Arc<Option<tobj::Material>>,
}

impl Draw for DrawData {
    fn draw(&self, modelmat: &Mat4, viewmat: &Mat4) {
        self.shader.use_program();
        self.mesh.lock().expect("Failed to lock mesh").get().bind();
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
        self.mesh.lock().expect("Failed to lock mesh").draw();
        unbind();
        unsafe { gl::UseProgram(0) };
    }
}
