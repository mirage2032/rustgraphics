use glengine::engine::drawable::mesh::{unbind, Mesh, MeshData};
use glengine::engine::drawable::Drawable;
use glengine::engine::drawable::mesh::model::ModelMesh;
use glengine::gl;
use glengine::glam::Mat4;

pub struct ArrayMesh {
    mesh: Box<dyn Mesh>,
    width: usize,
    height: usize,
    depth:usize,
    transforms: u32,
}

impl ArrayMesh {
    pub fn new(mesh:Box<dyn Mesh>,width: usize, height: usize,depth:usize) -> Self {
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
        // let mesh = Box::new(MeshData::new(&vertices, &normals, Some(&indices)));
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

            unbind();
        }

        Self {
            mesh,
            width,
            height,
            depth,
            transforms: transforms_vbo,
        }
    }
}

impl Drop for ArrayMesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.transforms);
        }
    }
}

impl Mesh for ArrayMesh {
    fn bind(&self) {
        self.mesh.bind();
    }
    fn get_indices_count(&self) -> u32 {
        self.mesh.get_indices_count()
    }
}

impl Drawable for ArrayMesh {
    fn draw(&self, modelmat: &Mat4, viewmat: &Mat4) {
        self.bind();
        unsafe {
            gl::DrawElementsInstanced(
                gl::TRIANGLES,
                self.mesh.get_indices_count() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
                (self.width * self.height * self.depth) as i32,
            );
        }
    }
}
