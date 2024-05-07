use glam::Mat4;
use std::sync::Arc;

use crate::engine::drawable::{Drawable, DrawObject};
use crate::engine::drawable::mesh::{Mesh, cube::CubeMesh, MeshTrait};
use crate::engine::shader::Shader;

pub struct BaseDrawable {
    draw_object: DrawObject
}

impl BaseDrawable {
    pub fn new(mesh:Arc<Box<dyn MeshTrait>>,shader: Arc<Shader>) -> Self {
        let draw_object = DrawObject{
            mesh,
            shader,
        };
        Self{draw_object}
    }
}

impl Drawable for BaseDrawable {
    fn draw(&self,modelmat: &Mat4, viewmat: &Mat4) {
        self.draw_object.draw(modelmat, viewmat);
    }
}

impl Default for BaseDrawable {
    fn default() -> Self {
        let draw_object = DrawObject{
            mesh: Arc::new(Box::new(CubeMesh::default())),
            shader: Arc::new(Shader::default()),
        };
        Self{draw_object}
    }
}