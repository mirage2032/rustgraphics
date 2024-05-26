use std::sync::{Arc, Mutex};

use glam::Mat4;

use shader::Shader;

use crate::engine::config::STATIC_DATA;
use crate::engine::drawable::mesh::unbind;
use crate::engine::scene::lights::Lights;

pub mod base;
pub mod importer;
pub mod material;
pub mod mesh;
pub mod shader;

pub trait Draw: Send {
    fn draw(&self, modelmat: &Mat4, viewmat: &Mat4, lights: &Lights);
}

pub struct DrawData {
    pub mesh: Arc<Mutex<dyn mesh::Mesh>>,
    pub shader: Arc<Shader>,
    pub material: Option<Arc<material::Material>>,
}

impl Draw for DrawData {
    fn draw(&self, modelmat: &Mat4, viewmat: &Mat4, lights: &Lights) {
        self.shader.use_program();

        lights.use_ssbo(0);
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

        if let Some(ref material) = self.material {
            material.set_uniforms(&self.shader);
        }
        self.mesh.lock().expect("Failed to lock mesh").draw();
        unbind();
        unsafe { gl::UseProgram(0) };
    }
}
