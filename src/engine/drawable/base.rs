use std::sync::{Arc, Mutex};

use glam::Mat4;

use crate::engine::drawable::{Draw, DrawData};
use crate::engine::drawable::mesh;

use super::shader::Shader;

pub struct Drawable {
    pub draw_data: Vec<DrawData>,
}

impl Drawable {
    pub fn new(mesh: Arc<Mutex<dyn mesh::Mesh>>, shader: Arc<Shader>) -> Self {
        let draw_object = DrawData {
            mesh,
            shader,
            material: Arc::new(None),
        };
        Self {
            draw_data: vec![draw_object],
        }
    }
}

impl Draw for Drawable {
    fn draw(&self, modelmat: &Mat4, viewmat: &Mat4) {
        for drawable in self.draw_data.iter() {
            drawable.draw(modelmat, viewmat);
        }
    }
}

impl Default for Drawable {
    fn default() -> Self {
        let draw_object = DrawData {
            mesh: mesh::cube::new(),
            shader: Arc::new(Shader::default()),
            material: Arc::new(None),
        };
        Self {
            draw_data: vec![draw_object],
        }
    }
}
