use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use glam::Mat4;
use shader::Shader;
use crate::engine::config::CONFIG;
use crate::engine::drawable::material::{Material, MaterialData, Texture, MATERIAL_MAP};
use crate::engine::drawable::mesh::{MeshData, MESH_MAP};
use crate::engine::drawable::shader::{ShaderType, SHADER_MAP};
use crate::engine::fbo::Fbo;
use crate::engine::scene::lights::Lights;

pub mod base;
pub mod importer;
pub mod material;
pub mod mesh;
pub mod shader;

pub trait Drawable{
    fn draw(&mut self, modelmat: &Mat4, viewmat: &Mat4, lights: Option<&Lights>);
}


#[derive(Copy,Clone,Debug)]
pub struct DrawData {
    pub mesh_id: usize,
    pub shader_type: ShaderType,
    pub material_id: Option<usize>,
}

impl Drawable for DrawData {
    fn draw(&mut self, modelmat: &Mat4, viewmat: &Mat4, lights: Option<&Lights>) {
        let shader = SHADER_MAP.get(&self.shader_type).expect("Shader not found");
        let mesh_map = MESH_MAP.lock().expect("Could not lock mesh map");
        let mesh = mesh_map.get(self.mesh_id).expect("Mesh not found");
        shader.use_program();
        shader.reset_texture_count();
        mesh.bind();
        shader.set_mat4("view_mat", viewmat);
        shader.set_mat4("model_mat", modelmat);
        let projection = {
            let data = *CONFIG.projection();
            data
        };
        shader.set_mat4("projection_mat", &projection);
        if let Some(material_id) = self.material_id {
            let material_map = MATERIAL_MAP.lock().expect("Could not lock material map");
            let material = material_map.get(material_id).expect("Material not found");
            material.set_uniforms(&shader);
        }
        if let Some(lights) = lights {
            lights.bind(5);
        }
        mesh.draw();
        Lights::unbind(5);
        MeshData::unbind();
        Shader::unbind();
    }
}

pub fn screenquad(fbo: &Fbo) -> DrawData {
    let mesh_id = mesh::screenquad::new();
    let shader = ShaderType::Included(shader::IncludedShaderType::UnlitQuad);
    let mut textures = HashMap::new();
    textures.insert("color_tex", Texture::new(fbo.color_texture, gl::TEXTURE_2D));
    textures.insert(
        "depth_stencil_tex",
        Texture::new(fbo.depth_stencil_texture, gl::TEXTURE_2D),
    );

    let material = Material {
        data: MaterialData::default(),
        textures,
    };
    let material_id = MATERIAL_MAP.lock().expect("Could not lock material map").add(material);
    DrawData {
        mesh_id,
        shader_type: shader,
        material_id: Some(material_id),
    }
}
