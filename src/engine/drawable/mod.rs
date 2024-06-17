use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use glam::Mat4;

use shader::Shader;

use crate::engine::config::CONFIG;
use crate::engine::drawable::material::{Material, MaterialData, Texture};
use crate::engine::drawable::mesh::unbind;
use crate::engine::drawable::shader::unlit::QUAD_SHADER;
use crate::engine::fbo::Fbo;
use crate::engine::scene::lights::Lights;

pub mod base;
pub mod importer;
pub mod material;
pub mod mesh;
pub mod shader;

pub trait Drawable: Send + Sync {
    fn draw(&self, modelmat: &Mat4, viewmat: &Mat4, lights: Option<&Lights>);
}

pub struct DrawData {
    pub mesh: Arc<Mutex<dyn mesh::Mesh>>,
    pub shader: Arc<Shader>,
    pub material: Option<Arc<material::Material>>,
}

impl Drawable for DrawData {
    fn draw(&self, modelmat: &Mat4, viewmat: &Mat4, lights: Option<&Lights>) {
        self.shader.use_program();

        self.mesh.lock().expect("Failed to lock mesh").get().bind();
        self.shader.set_mat4("view_mat", viewmat);
        self.shader.set_mat4("model_mat", modelmat);
        let projection = {
            let data = *CONFIG.read().expect("Failed to read config").projection();
            data
        };
        self.shader.set_mat4("projection_mat", &projection);
        if let Some(ref material) = self.material {
            material.set_uniforms(&self.shader);
        }
        if let Some(lights) = lights {
            lights.bind(5);
        }
        self.mesh.lock().expect("Failed to lock mesh").draw();
        Lights::unbind(5);
        unbind();

        unsafe {
            gl::UseProgram(0);
        };
    }
}

pub fn screenquad(fbo: &Fbo) -> DrawData {
    let mesh = mesh::screenquad::new();
    let shader = QUAD_SHADER.clone();
    let mut textures = HashMap::new();
    textures.insert("diffuse_texture", Texture { id: fbo.color_texture });
    textures.insert(
        "depth_texture",
        Texture {
            id: fbo.depth_texture,
        },
    );

    let material = Material {
        data: MaterialData::default(),
        textures,
    };
    DrawData {
        mesh,
        shader,
        material: Some(Arc::new(material)),
    }
}
