use std::sync::{Arc, Mutex};
use gl::types::GLuint;

use glam::Mat4;

use shader::Shader;

use crate::engine::config::CONFIG;
use crate::engine::drawable::mesh::unbind;
use crate::engine::scene::lights::Lights;
use crate::engine::drawable::material::{Material,MaterialData,Texture};

pub mod base;
pub mod importer;
pub mod material;
pub mod mesh;
pub mod shader;

pub trait Draw: Send {
    fn draw(&self, modelmat: &Mat4, viewmat: &Mat4, lights: Option<&Lights>);
}

pub struct DrawData {
    pub mesh: Arc<Mutex<dyn mesh::Mesh>>,
    pub shader: Arc<Shader>,
    pub material: Option<Arc<material::Material>>,
}

impl Draw for DrawData {
    fn draw(&self, modelmat: &Mat4, viewmat: &Mat4, lights: Option<&Lights>) {
        self.shader.use_program();

        self.mesh.lock().expect("Failed to lock mesh").get().bind();
        self.shader.set_mat4("view_mat", viewmat);
        self.shader.set_mat4("model_mat", modelmat);
        let projection = {
            let data = *CONFIG
                .read()
                .expect("Failed to read config")
                .projection();
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

pub fn screenquad(size_x:usize,size_y:usize,texture:GLuint) ->DrawData{
    let mesh = mesh::screenquad::new(size_x,size_y);
    let shader = Arc::new(shader::new_quad_shader().expect("Could not create quad shader"));
    let material = Material{
        data: MaterialData::default(),
        diffuse_texture:Some(Texture{id:texture}),
    };
    DrawData {
        mesh,
        shader,
        material:Some(Arc::new(material)),
    }
    
}