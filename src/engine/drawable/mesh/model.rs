use std::path::Path;

use tobj::load_obj;

use crate::engine::drawable::mesh::{Mesh, MeshData};

pub struct ModelMesh {
    mesh: MeshData,
}
impl ModelMesh {
    pub fn new<P: AsRef<Path> + std::fmt::Debug>(path: P) -> Self {
        let (models, _) =
            load_obj(path, &tobj::LoadOptions::default()).expect("Failed to load obj file");
        let mut vertices: Vec<f32> = vec![];
        let mut normals: Vec<f32> = vec![];
        let mut indices: Vec<u32> = vec![];
        let mut vertex_offset: usize = 0;

        for model in models.iter() {
            let mesh = &model.mesh;

            vertices.extend_from_slice(&mesh.positions);
            normals.extend_from_slice(&mesh.normals);

            // Merge indices with correct offset
            for &index in &mesh.indices {
                // Adjust index based on total vertices processed
                indices.push(index as u32 + vertex_offset as u32);
            }

            // Update vertex offset
            vertex_offset += mesh.positions.len() / 3;
        }

        Self {
            mesh: MeshData::new(&vertices, &normals, Some(&indices)),
        }
    }
}

impl Mesh for ModelMesh {
    fn bind(&self) {
        self.mesh.bind();
    }
    fn get_indices_count(&self) -> u32 {
        self.mesh.indices_count
    }
    fn draw(&self) {
        self.mesh.draw();
    }
}
