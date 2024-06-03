use std::sync::{Arc, Mutex};

use glam::Mat4;

use crate::engine::drawable::{Drawable, DrawData};
use crate::engine::drawable::mesh;
use crate::engine::scene::lights::Lights;

use super::shader::Shader;

pub struct BaseDrawable {
    pub draw_data: Vec<DrawData>,
}

impl BaseDrawable {
    pub fn new(mesh: Arc<Mutex<dyn mesh::Mesh>>, shader: Arc<Shader>) -> Self {
        let draw_object = DrawData {
            mesh,
            shader,
            material: None,
        };
        Self {
            draw_data: vec![draw_object],
        }
    }
}

impl Drawable for BaseDrawable {
    fn draw(&self, modelmat: &Mat4, viewmat: &Mat4,lights:Option<&Lights>) {
        for drawable in self.draw_data.iter() {
            drawable.draw(modelmat, viewmat,lights);
        }
    }
}

impl Default for BaseDrawable {
    fn default() -> Self {
        let draw_object = DrawData {
            mesh: mesh::cube::new(),
            shader: Arc::new(Shader::default()),
            material: None,
        };
        Self {
            draw_data: vec![draw_object],
        }
    }
}
