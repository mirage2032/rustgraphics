use glam::Mat4;

use crate::engine::drawable::Drawable;

pub mod cube;
pub mod model;

pub trait Mesh: Drawable {
    fn bind(&self);
}
pub struct MeshData {
    vao: u32,
    vbo_vertices: u32,
    vbo_normals: u32,
    ebo: Option<u32>,
    indices_count: i32,
}

impl MeshData {
    pub fn new(vertices: &[f32], normals: &[f32], indices: Option<&[u32]>) -> Self {
        let mut vao = 0;
        let mut vbo_vertices = 0;
        let mut vbo_normals = 0;
        let mut ebo = 0;
        let indices_count = indices
            .map(|i| i.len() as i32)
            .unwrap_or(vertices.len() as i32 / 3);

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo_vertices);
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo_vertices);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::GenBuffers(1, &mut vbo_normals);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo_normals);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (normals.len() * std::mem::size_of::<f32>()) as isize,
                normals.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::EnableVertexAttribArray(1);

            if let Some(i) = indices {
                gl::GenBuffers(1, &mut ebo);
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
                gl::BufferData(
                    gl::ELEMENT_ARRAY_BUFFER,
                    (i.len() * std::mem::size_of::<u32>()) as isize,
                    i.as_ptr() as *const _,
                    gl::STATIC_DRAW,
                );
            }
        }
        Self {
            vao,
            vbo_vertices,
            vbo_normals,
            ebo: if indices.is_some() { Some(ebo) } else { None },
            indices_count,
        }
    }
}

impl Drop for MeshData {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo_vertices);
            gl::DeleteBuffers(1, &self.vbo_normals);
            if let Some(ebo) = self.ebo {
                gl::DeleteBuffers(1, &ebo);
            };
        }
    }
}

impl Mesh for MeshData {
    fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }
}

fn unbind() {
    unsafe {
        gl::BindVertexArray(0);
    }
}

impl Drawable for MeshData {
    fn draw(&self, _: &Mat4, _: &Mat4) {
        self.bind();
        if let Some(ebo) = self.ebo {
            unsafe {
                gl::DrawElements(
                    gl::TRIANGLES,
                    self.indices_count,
                    gl::UNSIGNED_INT,
                    std::ptr::null(),
                );
            }
        } else {
            unsafe {
                gl::DrawArrays(gl::TRIANGLES, 0, self.indices_count);
            }
        }
        unbind();
    }
}
