use std::cell::RefCell;
use std::rc::Rc;

use glam::Mat4;

use crate::engine::drawable::{mesh, Drawable};
use crate::engine::drawable::DrawData;
use crate::engine::scene::lights::Lights;

use super::shader::Shader;

pub struct BaseDrawable {
    pub draw_data: Vec<DrawData>,
}

impl BaseDrawable {
    pub fn new(mesh: Rc<RefCell<dyn mesh::Mesh>>, shader: Rc<RefCell<Shader>>) -> Self {
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
    fn draw(&mut self, modelmat: &Mat4, viewmat: &Mat4,lights:Option<&Lights>) {
        for drawable in self.draw_data.iter_mut() {
            drawable.draw(modelmat, viewmat,lights);
        }
    }
}

impl Default for BaseDrawable {
    fn default() -> Self {
        let draw_object = DrawData {
            mesh: mesh::cube::new(),
            shader: Rc::new(RefCell::new(Shader::default())),
            material: None,
        };
        Self {
            draw_data: vec![draw_object],
        }
    }
}
