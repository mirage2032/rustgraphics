use std::fs::File;
use std::io::BufReader;
use std::ops::Sub;
use std::path::Path;

use glam::Mat4;
use tobj::{load_obj};

use crate::engine::drawable::Drawable;
use crate::engine::drawable::mesh::{Mesh, MeshData};

pub struct ModelMesh {
    mesh: MeshData,
}
impl ModelMesh {
    pub fn new<P: AsRef<Path> + std::fmt::Debug>(path: P) -> Self {
        let (models,materials) = load_obj(path,&tobj::LoadOptions::default()).expect("Failed to load obj file");
        let mut vertices: Vec<f32> = vec![];
        let mut indices: Vec<u32> = vec![];
        let mut vertex_offset: usize = 0;

        for model in models.iter() {
            let mesh = &model.mesh;

            // Merge positions
            vertices.extend_from_slice(&mesh.positions);

            // Merge indices with correct offset
            for &index in &mesh.indices {
                // Adjust index based on total vertices processed
                indices.push(index as u32 + vertex_offset as u32);
            }

            // Update vertex offset
            vertex_offset += mesh.positions.len() / 3;
        }
        
        let mut normals: Vec<f32> = vec![];
        let mut x_range = (f32::MAX, f32::MIN);
        let mut y_range = (f32::MAX, f32::MIN);
        let mut z_range = (f32::MAX, f32::MIN);
        for vertex in vertices.chunks_mut(3) {
            let x = vertex[0];
            let y = vertex[1];
            let z = vertex[2];
            if x < x_range.0 {
                x_range.0 = x;
            }
            if x > x_range.1 {
                x_range.1 = x;
            }
            if y < y_range.0 {
                y_range.0 = y;
            }
            if y > y_range.1 {
                y_range.1 = y;
            }
            if z < z_range.0 {
                z_range.0 = z;
            }
            if z > z_range.1 {
                z_range.1 = z;
            }
        }
        //iterate in 6 long chunks
        for vertex in vertices.chunks_mut(3) {
            let normal_x = (vertex[0] - x_range.0) / (x_range.1 - x_range.0);
            let normal_y = (vertex[1] - y_range.0) / (y_range.1 - y_range.0);
            let normal_z = (vertex[2] - z_range.0) / (z_range.1 - z_range.0);
            normals.push(normal_x);
            normals.push(normal_y);
            normals.push(normal_z);
        }

        Self {
            mesh: MeshData::new(&vertices,&normals, Some(&indices)),
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
}

impl Drawable for ModelMesh {
    fn draw(&self, modelmat: &Mat4, viewmat: &Mat4) {
        self.mesh.draw(modelmat, viewmat);
    }
}
