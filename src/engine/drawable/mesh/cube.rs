use nalgebra_glm::TMat4;
use crate::engine::drawable::Drawable;
use crate::engine::drawable::mesh::{Mesh, MeshTrait};

pub struct CubeMesh {
    mesh: Mesh,
}

impl MeshTrait for CubeMesh {
    fn bind(&self) {
        self.mesh.bind();
    }
}

impl Drawable for CubeMesh{
    fn draw(&self,modelmat: &TMat4<f32>, viewmat: &TMat4<f32>) {
        self.mesh.draw(modelmat, viewmat);
    }
}
impl Default for CubeMesh{
    fn default() -> Self {
        let vertices: [f32; 144] = [
            // Front face
            -0.5, -0.5,  0.5,  1.0, 0.0, 0.0, // Bottom-left
            0.5, -0.5,  0.5,  1.0, 0.0, 0.0, // Bottom-right
            0.5,  0.5,  0.5,  1.0, 0.0, 0.0, // Top-right
            -0.5,  0.5,  0.5,  1.0, 0.0, 0.0, // Top-left
            // Back face
            -0.5, -0.5, -0.5,  0.0, 1.0, 0.0, // Bottom-left
            0.5, -0.5, -0.5,  0.0, 1.0, 0.0, // Bottom-right
            0.5,  0.5, -0.5,  0.0, 1.0, 0.0, // Top-right
            -0.5,  0.5, -0.5,  0.0, 1.0, 0.0, // Top-left
            // Top face
            0.5,  0.5,  0.5,  0.0, 0.0, 1.0, // Front-right
            -0.5,  0.5,  0.5,  0.0, 0.0, 1.0, // Front-left
            -0.5,  0.5, -0.5,  0.0, 0.0, 1.0, // Back-left
            0.5,  0.5, -0.5,  0.0, 0.0, 1.0, // Back-right
            // Bottom face
            -0.5, -0.5,  0.5,  1.0, 1.0, 0.0, // Front-left
            0.5, -0.5,  0.5,  1.0, 1.0, 0.0, // Front-right
            0.5, -0.5, -0.5,  1.0, 1.0, 0.0, // Back-right
            -0.5, -0.5, -0.5,  1.0, 1.0, 0.0, // Back-left
            // Right face
            0.5, -0.5,  0.5,  1.0, 0.0, 1.0, // Front-bottom
            0.5,  0.5,  0.5,  1.0, 0.0, 1.0, // Front-top
            0.5,  0.5, -0.5,  1.0, 0.0, 1.0, // Back-top
            0.5, -0.5, -0.5,  1.0, 0.0, 1.0, // Back-bottom
            // Left face
            -0.5, -0.5,  0.5,  0.0, 1.0, 1.0, // Front-bottom
            -0.5,  0.5,  0.5,  0.0, 1.0, 1.0, // Front-top
            -0.5,  0.5, -0.5,  0.0, 1.0, 1.0, // Back-top
            -0.5, -0.5, -0.5,  0.0, 1.0, 1.0, // Back-bottom
        ];

        let indices: [u32; 36] = [
            0, 1, 2, 2, 3, 0, // Front face
            4, 5, 6, 6, 7, 4, // Back face
            8, 9, 10, 10, 11, 8, // Top face
            12, 13, 14, 14, 15, 12, // Bottom face
            16, 17, 18, 18, 19, 16, // Right face
            20, 21, 22, 22, 23, 20, // Left face
        ];

        let (vbo, vao, ebo) = unsafe {
            let mut vbo = 0;
            let mut vao = 0;
            let mut ebo = 0;

            gl::GenBuffers(1, &mut vbo);
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER,
                           (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                           &vertices[0] as *const f32 as *const gl::types::GLvoid,
                           gl::STATIC_DRAW);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                           (indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                           &indices[0] as *const u32 as *const gl::types::GLvoid,
                           gl::STATIC_DRAW);

            let stride = 6 * std::mem::size_of::<f32>() as gl::types::GLsizei;
            let offset = 0 as *const gl::types::GLvoid;

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, offset);
            gl::EnableVertexAttribArray(0);

            let color_offset = 3 * std::mem::size_of::<f32>() as gl::types::GLsizei;
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, color_offset as *const gl::types::GLvoid);
            gl::EnableVertexAttribArray(1);

            (vbo, vao, ebo)
        };
        Self {mesh: Mesh {vao, vbo, ebo, indices_count: indices.len() as i32}}
    }
}