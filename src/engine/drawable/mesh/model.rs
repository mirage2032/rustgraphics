use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use glam::Mat4;
use obj::{load_obj, Obj};

use crate::engine::drawable::Drawable;
use crate::engine::drawable::mesh::{Mesh, MeshData};

pub struct ModelMesh {
    mesh: MeshData,
}
impl ModelMesh {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let input = BufReader::new(File::open(path).unwrap());
        let dome: Obj = load_obj(input).expect("Failed to load obj file");
        let mut vertices: Vec<f32> = vec![];
        let mut normals: Vec<f32> = vec![];
        let mut x_range = (f32::MAX, f32::MIN);
        let mut y_range = (f32::MAX, f32::MIN);
        let mut z_range = (f32::MAX, f32::MIN);
        for vertex in dome.vertices.iter() {
            let x = vertex.position[0];
            let y = vertex.position[1];
            let z = vertex.position[2];
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
            vertices.push(x);
            vertices.push(y);
            vertices.push(z);
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

        let indices: Vec<u32> = dome.indices.iter().map(|i| *i as u32).collect();

        Self {
            mesh: MeshData::new(&vertices,&normals, Some(&indices)),
        }
    }
}

impl Mesh for ModelMesh {
    fn bind(&self) {
        self.mesh.bind();
    }
}

impl Drawable for ModelMesh {
    fn draw(&self, modelmat: &Mat4, viewmat: &Mat4) {
        self.mesh.draw(modelmat, viewmat);
    }
}
