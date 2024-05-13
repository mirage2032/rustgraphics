use glengine::engine::drawable::mesh::{Mesh, MeshData};
use glengine::gl;

pub struct InstancedMesh {
    mesh_data: MeshData,
    width: usize,
    height: usize,
    depth: usize,
    transforms: u32,
}

impl InstancedMesh {
    pub fn new(mesh: MeshData, width: usize, height: usize, depth: usize) -> Self {
        let mut transforms_all: Vec<f32> = vec![];
        let length = 50;
        let scale = 5.0 / length as f32;
        let offset = scale * 30.0;
        for offset_z in 0..depth {
            for offset_y in 0..height {
                for offset_x in 0..width {
                    transforms_all.push(offset * (offset_x as f32 - (width as f32 / 2.0)));
                    transforms_all.push(offset * (offset_y as f32 - (height as f32 / 2.0)));
                    transforms_all.push(offset * (offset_z as f32 - (depth as f32 / 2.0)))
                }
            }
        }
        mesh.bind();

        let mut transforms_vbo = 0;
        unsafe {
            gl::GenBuffers(1, &mut transforms_vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, transforms_vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (transforms_all.len() * std::mem::size_of::<f32>()) as isize,
                transforms_all.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            let attrib_index = 3;
            let stride = 3 * std::mem::size_of::<f32>() as gl::types::GLsizei;
            gl::VertexAttribPointer(
                attrib_index,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(attrib_index);
            gl::VertexAttribDivisor(attrib_index, 1);

            mesh.unbind();
        }

        Self {
            mesh_data: mesh,
            width,
            height,
            depth,
            transforms: transforms_vbo,
        }
    }
}

impl Drop for InstancedMesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.transforms);
        }
    }
}

impl Mesh for InstancedMesh {
    fn get(&self) -> &MeshData {
        &self.mesh_data
    }
    fn get_mut(&mut self) -> &mut MeshData {
        &mut self.mesh_data
    }
    fn draw(&self) {
        self.get().bind();
        unsafe {
            gl::DrawElementsInstanced(
                gl::TRIANGLES,
                self.mesh_data.get_indices_count() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
                (self.width * self.height * self.depth) as i32,
            );
        }
        self.get().unbind();
    }
}
