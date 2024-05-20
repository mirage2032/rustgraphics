use std::path::Path;
use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;
use tobj::{load_mtl, load_obj};

use crate::engine::drawable::base::Drawable;
use crate::engine::drawable::DrawData;
use crate::engine::drawable::mesh::{BaseMesh, MeshData};
use crate::engine::drawable::shader::color::{new_lit_color_shader,new_unlit_color_shader};
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
    let mut drawables: Vec<DrawData> = vec![];
    lazy_static! {
        static ref NO_SHADER: Arc<Shader> = Arc::new(Shader::default());
    }
    for model in models.iter() {
        let material = match (&mtl_data, &model.mesh.material_id) {
            (Some((ref materials, _)), Some(material_id)) => Some(&materials[*material_id]),
            _ => None,
        };
        let mesh_data = MeshData::new(&model.mesh.positions)
            .with_normals(&model.mesh.normals)
            .with_indices(&model.mesh.indices)
            .with_texcoords(&model.mesh.texcoords);
        let mesh = BaseMesh { mesh_data };
        let shader = match material {
            Some(material) => {
                if let Some(ref albedo_texture) = material.diffuse {
                    Arc::new(new_unlit_color_shader(albedo_texture))
                } else {
                    NO_SHADER.clone()
                }
            }
            None => NO_SHADER.clone(),
        };
        drawables.push(DrawData {
            mesh: Arc::new(Mutex::new(mesh)),
            shader,
            material: Arc::new(material.cloned()),
        })
    }

    Drawable {
        draw_data: drawables,
    }
}
