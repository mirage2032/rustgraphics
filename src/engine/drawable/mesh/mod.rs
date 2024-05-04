pub mod cube;

use nalgebra_glm::TMat4;
use crate::engine::drawable::Drawable;

pub trait MeshTrait: Drawable {
    fn bind(&self);
}
pub struct Mesh {
    vao: u32,
    vbo: u32,
    ebo: u32,
    indices_count: i32,
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
        }
    }
}

impl MeshTrait for Mesh {
    fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
        }
    }
}

impl Drawable for Mesh {
    fn draw(&self,_: &TMat4<f32>, _: &TMat4<f32>) {
        self.bind();
        unsafe {
            gl::DrawElements(gl::TRIANGLES, self.indices_count, gl::UNSIGNED_INT, std::ptr::null());
        }
    }
}