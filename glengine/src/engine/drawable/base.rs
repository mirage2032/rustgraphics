use std::cell::RefCell;
use std::rc::Rc;

use glam::Mat4;

use crate::engine::drawable::{mesh, Drawable};
use crate::engine::drawable::DrawData;
use crate::engine::scene::lights::Lights;

use super::shader::{IncludedShaderType, Shader, ShaderType};

#[derive(Clone,Debug)]
pub struct BaseDrawable {
    pub draw_data: Vec<DrawData>,
}

impl BaseDrawable {
    pub fn new(mesh_id: usize, shader_type: ShaderType) -> Self {
        let draw_object = DrawData {
            mesh_id,
            shader_type,
            material_id: None,
        };
        Self {
            draw_data: vec![draw_object],
        }
    }
}

impl Drawable for BaseDrawable {
    fn draw(&mut self, modelmat: &Mat4, viewmat: &Mat4,lights:Option<&Lights>) {
        for drawable in self.draw_data.iter_mut() {
            drawable.draw(modelmat, viewmat,lights);
        }
    }
}

impl Default for BaseDrawable {
    fn default() -> Self {
        let draw_object = DrawData {
            mesh_id: mesh::cube::new(),
            shader_type: ShaderType::Included(IncludedShaderType::Basic),
            material_id: None,
        };
        Self {
            draw_data: vec![draw_object],
        }
    }
}
