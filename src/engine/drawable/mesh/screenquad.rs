use std::sync::{Arc, Mutex};

use crate::engine::drawable::mesh::*;

//for the quad that will be used to render the screen
pub fn new(size_x:usize,size_y:usize) -> Arc<Mutex<dyn Mesh>> {
    let vertices = vec![
        -1.0, -1.0, 0.0, // bottom left
        1.0, -1.0, 0.0, // bottom right
        1.0, 1.0, 0.0, // top right
        -1.0, 1.0, 0.0, // top left
    ];
    
    let indices = vec![
        0, 1, 2,
        2, 3, 0,
    ];
    
    let tex_coords = vec![
        0.0, 0.0, // bottom left
        1.0, 0.0, // bottom right
        1.0, 1.0, // top right
        0.0, 1.0, // top left
    ];
    let mesh_data = MeshData::new(&vertices)
        .with_indices(&indices)
        .with_texcoords(&tex_coords);
    Arc::new(Mutex::new(BaseMesh { mesh_data }))
}