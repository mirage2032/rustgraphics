use crate::engine::drawable::manager::DRAWABLE_MANAGER;
use crate::engine::drawable::mesh::manager::MeshHandle;
use crate::engine::drawable::mesh::{BaseMesh, MeshData};

//for the quad that will be used to render the screen
pub fn new() -> MeshHandle {
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
    DRAWABLE_MANAGER.with(|dm|dm.borrow_mut().mesh.add(Box::new(BaseMesh { mesh_data })))
    
}