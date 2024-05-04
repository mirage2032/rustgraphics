use glam::Mat4;
use crate::engine::drawable::{Drawable, DrawObject};
use crate::engine::drawable::mesh::cube::CubeMesh;
use crate::engine::shader::Shader;
use crate::engine::transform::Transform;

pub struct DrawCube {
    draw_object: DrawObject
}

impl Drawable for DrawCube {
    fn draw(&self,modelmat: &Mat4, viewmat: &Mat4) {
        self.draw_object.draw(modelmat, viewmat);
    }
}

impl Default for DrawCube {
    fn default() -> Self {
        let shader = Shader::default();
        let draw_object = DrawObject{
            mesh: Box::new(CubeMesh::default()),
            shader: Shader::default(),
        };
        Self{draw_object}
    }
}