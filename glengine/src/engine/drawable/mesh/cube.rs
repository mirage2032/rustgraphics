use std::cell::RefCell;
use std::rc::Rc;

use crate::engine::drawable::mesh::*;

pub fn new() -> Rc<RefCell<dyn Mesh>> {
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
        // Front face
        0.0, 0.0, 1.0, // Bottom-left   - 0
        0.0, 0.0, 1.0, // Bottom-right  - 1
        0.0, 0.0, 1.0, // Top-right     - 2
        0.0, 0.0, 1.0, // Top-left      - 3
        // Back face
        0.0, 0.0, -1.0, // Bottom-left  - 4
        0.0, 0.0, -1.0, // Bottom-right - 5
        0.0, 0.0, -1.0, // Top-right    - 6
        0.0, 0.0, -1.0, // Top-left     - 7
        // Top face
        0.0, 1.0, 0.0, // Front-left   - 8
        0.0, 1.0, 0.0, // Front-right  - 9
        0.0, 1.0, 0.0, // Back-right   - 10
        0.0, 1.0, 0.0, // Back-left    - 11
        // Bottom face
        0.0, -1.0, 0.0, // Front-left  - 12
        0.0, -1.0, 0.0, // Front-right - 13
        0.0, -1.0, 0.0, // Back-right  - 14
        0.0, -1.0, 0.0, // Back-left   - 15
        // Right face
        1.0, 0.0, 0.0, // Front-bottom - 16
        1.0, 0.0, 0.0, // Front-top    - 17
        1.0, 0.0, 0.0, // Back-top     - 18
        1.0, 0.0, 0.0, // Back-bottom  - 19
        // Left face
        -1.0, 0.0, 0.0, // Front-bottom - 20
        -1.0, 0.0, 0.0, // Front-top    - 21
        -1.0, 0.0, 0.0, // Back-top     - 22
        -1.0, 0.0, 0.0, // Back-bottom  - 23
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
    let mesh_data = MeshData::new(&vertices)
        .with_normals(&normals)
        .with_indices(&indices);
    Rc::new(RefCell::new(BaseMesh { mesh_data }))
}
