use std::collections::HashMap;
use glam::Mat4;
use shader::Shader;
use crate::engine::config::CONFIG;
use crate::engine::drawable::manager::DRAWABLE_MANAGER;
use crate::engine::drawable::material::{Material, MaterialData, Texture, manager::MaterialHandle};
use crate::engine::drawable::mesh::{MeshData, manager::MeshHandle};
use crate::engine::drawable::shader::manager::ShaderHandle;
use crate::engine::fbo::Fbo;
use crate::engine::scene::lights::Lights;

pub mod base;
pub mod importer;
pub mod material;
pub mod mesh;
pub mod shader;
pub mod manager;

pub trait Drawable{
    fn draw(&mut self, modelmat: &Mat4, viewmat: &Mat4, lights: Option<&Lights>);
}


#[derive(Clone)]
pub struct DrawData {
    pub mesh_handle: MeshHandle,
    pub shader_handle: ShaderHandle,
    pub material_handle: Option<MaterialHandle>,
}

impl Drawable for DrawData {
    fn draw(&mut self, modelmat: &Mat4, viewmat: &Mat4, lights: Option<&Lights>) {
        let projection = *CONFIG.projection();
        DRAWABLE_MANAGER.with(|dm|
            {
                let draw_manager = dm.borrow();
                let shader = draw_manager.shader.get(&self.shader_handle).expect("Shader not found");
                shader.use_program();
                shader.reset_texture_count();
                shader.set_mat4("view_mat", viewmat);
                shader.set_mat4("model_mat", modelmat);
                shader.set_mat4("projection_mat", &projection);
                if let Some(material_id) = &self.material_handle {
                    draw_manager.material.get(&material_id).expect("Material not found").set_uniforms(&shader); 
                }
        if let Some(lights) = lights {
            lights.bind(5);
        }
        draw_manager.mesh.get(&self.mesh_handle).expect("Mesh not found").bind();
        draw_manager.mesh.get(&self.mesh_handle).expect("Mesh not found").draw();
            }
        );
        Lights::unbind(5);
        MeshData::unbind();
        Shader::unbind();
    }
}

pub fn screenquad(fbo: &Fbo) -> DrawData {
    let mesh_id = mesh::screenquad::new();
    let shader_handle = shader::manager::IncludedShaderHandle::UnlitQuad.into();
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
    let material_id = DRAWABLE_MANAGER.with(|dm|dm.borrow_mut().material.add(material));
    DrawData {
        mesh_handle: mesh_id,
        shader_handle,
        material_handle: Some(material_id),
    }
}
