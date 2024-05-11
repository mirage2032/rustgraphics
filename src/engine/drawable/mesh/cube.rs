use crate::engine::drawable::mesh::{Mesh, MeshData};

pub struct CubeMesh {
    mesh: MeshData,
}

impl Mesh for CubeMesh {
    fn bind(&self) {
        self.mesh.bind();
    }
    fn get_indices_count(&self) -> u32 {
        self.mesh.get_indices_count()
    }
    fn draw(&self) {
        self.mesh.draw();
    }
}

impl Default for CubeMesh {
    fn default() -> Self {
        let vertices: [f32; 72] = [
            // Front face
            -0.5, -0.5, 0.5, // Bottom-left   - 0
            0.5, -0.5, 0.5, // Bottom-right   - 1
            0.5, 0.5, 0.5, // Top-right      - 2
            -0.5, 0.5, 0.5, // Top-left      - 3
            // Back face
            -0.5, -0.5, -0.5, // Bottom-left  - 4
            0.5, -0.5, -0.5, // Bottom-right   - 5
            0.5, 0.5, -0.5, // Top-right      - 6
            -0.5, 0.5, -0.5, // Top-left      - 7
            // Top face
            -0.5, 0.5, 0.5, // Front-left   - 8
            0.5, 0.5, 0.5, // Front-right   - 9
            0.5, 0.5, -0.5, // Back-right      - 10
            -0.5, 0.5, -0.5, // Back-left     - 11
            // Bottom face
            -0.5, -0.5, 0.5, // Front-left   - 12
            0.5, -0.5, 0.5, // Front-right    - 13
            0.5, -0.5, -0.5, // Back-right     - 14
            -0.5, -0.5, -0.5, // Back-left    - 15
            // Right face
            0.5, -0.5, 0.5, // Front-bottom  - 16
            0.5, 0.5, 0.5, // Front-top     - 17
            0.5, 0.5, -0.5, // Back-top       - 18
            0.5, -0.5, -0.5, // Back-bottom    - 19
            // Left face
            -0.5, -0.5, 0.5, // Front-bottom - 20
            -0.5, 0.5, 0.5, // Front-top     - 21
            -0.5, 0.5, -0.5, // Back-top       - 22
            -0.5, -0.5, -0.5, // Back-bottom    - 23
        ];

        let normals: [f32; 72] = [
            1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0,
            0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0,
            0.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0,
            1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0,
            1.0, 0.0, 1.0, 1.0,
        ];

        let indices: [u32; 36] = [
            // Front face
            0, 1, 2, 2, 3, 0, // Back face
            6, 5, 4, 4, 7, 6, // Top face
            8, 9, 10, 10, 11, 8, // Bottom face
            14, 13, 12, 12, 15, 14, // Right face
            18, 17, 16, 16, 19, 18, // Left face
            20, 21, 22, 22, 23, 20,
        ];

        Self {
            mesh: MeshData::new(&vertices, &normals, Some(&indices)),
        }
    }
}
