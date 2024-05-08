use glam::Mat4;
use crate::engine::drawable::Drawable;
use crate::engine::drawable::mesh::{MeshData, Mesh};

pub struct CubeMesh {
    mesh: MeshData,
}

impl Mesh for CubeMesh {
    fn bind(&self) {
        self.mesh.bind();
    }
}

impl Drawable for CubeMesh{
    fn draw(&self,modelmat: &Mat4, viewmat: &Mat4) {
        self.mesh.draw(modelmat, viewmat);
    }
}
impl Default for CubeMesh{
    fn default() -> Self {
        let vertices: [f32; 72] = [
            // Front face
            -0.5, -0.5,  0.5, // Bottom-left
            0.5, -0.5,  0.5, // Bottom-right
            0.5,  0.5,  0.5, // Top-right
            -0.5,  0.5,  0.5, // Top-left
            // Back face
            -0.5, -0.5, -0.5, // Bottom-left
            0.5, -0.5, -0.5, // Bottom-right
            0.5,  0.5, -0.5, // Top-right
            -0.5,  0.5, -0.5, // Top-left
            // Top face
            0.5,  0.5,  0.5, // Front-right
            -0.5,  0.5,  0.5, // Front-left
            -0.5,  0.5, -0.5, // Back-left
            0.5,  0.5, -0.5, // Back-right
            // Bottom face
            -0.5, -0.5,  0.5, // Front-left
            0.5, -0.5,  0.5, // Front-right
            0.5, -0.5, -0.5, // Back-right
            -0.5, -0.5, -0.5, // Back-left
            // Right face
            0.5, -0.5,  0.5, // Front-bottom
            0.5,  0.5,  0.5, // Front-top
            0.5,  0.5, -0.5, // Back-top
            0.5, -0.5, -0.5, // Back-bottom
            // Left face
            -0.5, -0.5,  0.5, // Front-bottom
            -0.5,  0.5,  0.5, // Front-top
            -0.5,  0.5, -0.5, // Back-top
            -0.5, -0.5, -0.5, // Back-bottom
        ];
        
        let normals: [f32; 72] = [
            1.0, 0.0, 0.0,
            1.0, 0.0, 0.0,
            1.0, 0.0, 0.0,
            1.0, 0.0, 0.0,
            
            0.0, 1.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 1.0, 0.0,
            
            0.0, 0.0, 1.0,
            0.0, 0.0, 1.0,
            0.0, 0.0, 1.0,
            0.0, 0.0, 1.0,
            
            1.0, 1.0, 0.0,
            1.0, 1.0, 0.0,
            1.0, 1.0, 0.0,
            1.0, 1.0, 0.0,
            
            1.0, 0.0, 1.0,
            1.0, 0.0, 1.0,
            1.0, 0.0, 1.0,
            1.0, 0.0, 1.0,
            
            0.0, 1.0, 1.0,
            0.0, 1.0, 1.0,
            0.0, 1.0, 1.0,
            0.0, 1.0, 1.0
            ];

        let indices: [u32; 36] = [
            0, 1, 2, 2, 3, 0, // Front face
            4, 5, 6, 6, 7, 4, // Back face
            8, 9, 10, 10, 11, 8, // Top face
            12, 13, 14, 14, 15, 12, // Bottom face
            16, 17, 18, 18, 19, 16, // Right face
            20, 21, 22, 22, 23, 20, // Left face
        ];

        Self {mesh: MeshData::new(&vertices,&normals, Some(&indices))}
    }
}