use crate::drawable::{Drawable, DrawObject};
use crate::drawable::mesh::CubeMesh;
use crate::drawable::transform::Transform;
use crate::shader::Shader;

struct DrawCube {
    draw_object: DrawObject
}

impl Drawable for DrawCube {
    fn draw(&self) {
        self.draw_object.draw();
    }
}

impl Default for DrawCube {
    fn default() -> Self {
        let shader = Shader::default();
        let draw_object = DrawObject{
            mesh: Box::new(CubeMesh::default()),
            shader: Shader::default(),
            transform: Transform::default(),
        };
        Self{draw_object}
    }
}