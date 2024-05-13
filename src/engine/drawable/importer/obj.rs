use std::path::Path;
use std::sync::{Arc, Mutex};

use tobj::{load_mtl, load_obj};

use crate::engine::drawable::base::Drawable;
use crate::engine::drawable::DrawData;
use crate::engine::drawable::mesh::{MeshData, BaseMesh};
use crate::engine::drawable::shader::Shader;

pub fn import<P: AsRef<Path> + std::fmt::Debug>(path: P) -> Drawable {
    let (models, _) = load_obj(&path, &tobj::GPU_LOAD_OPTIONS).expect("Failed to load obj file"); //TODO: handle error

    let mtl_path = path.as_ref().with_extension("mtl");
    let mtl_data = if mtl_path.exists() {
        match load_mtl(mtl_path) {
            Ok(data) => Some(data),
            Err(_) => None,
        }
    } else {
        None
    };
    let mut vertices: Vec<f32> = vec![];
    let mut drawables: Vec<DrawData> = vec![];

    for model in models.iter() {
        let material = match (&mtl_data, &model.mesh.material_id) {
            (Some((ref materials, _)), Some(material_id)) => Some(&materials[*material_id]),
            _ => None,
        };
        let mesh_data = MeshData::new(&model.mesh.positions)
            .with_normals(&model.mesh.normals)
            .with_indices(&model.mesh.indices)
            .with_texcoords(&model.mesh.texcoords);
        let mesh = BaseMesh{ mesh_data};
        drawables.push(DrawData {
            mesh: Arc::new(Mutex::new(mesh)),
            shader: Arc::new(Shader::default()),
        })
    }

    Drawable {
        draw_data: drawables,
    }
}
