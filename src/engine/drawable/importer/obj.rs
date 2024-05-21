use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;
use tobj::{load_mtl, load_obj};

use crate::engine::drawable::base::Drawable;
use crate::engine::drawable::mesh::{BaseMesh, MeshData};
use crate::engine::drawable::shader::color::new_unlit_color_shader;
use crate::engine::drawable::shader::Shader;
use crate::engine::drawable::DrawData;
use crate::engine::drawable::material::Material;

lazy_static! {
    static ref NO_SHADER: Arc<Shader> = Arc::new(Shader::default());
}
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
    let mut arc_materials: HashMap<usize, (Arc<Material>,&tobj::Material)> = HashMap::new();
    for model in models.iter() {
        let material = match (&mtl_data, &model.mesh.material_id) {
            (Some((materials, _)), Some(material_id)) => match arc_materials.get(material_id) {
                Some(material) => Some(material.clone()),
                None => {
                    let tobj_mat = &materials[*material_id];
                    let gl_mat =Arc::new(Material::from( tobj_mat.clone()));
                    arc_materials.insert(*material_id, (gl_mat.clone(),tobj_mat));
                    Some((gl_mat,tobj_mat))
                }
            },
            _ => None,
        };
        let mesh_data = MeshData::new(&model.mesh.positions)
            .with_normals(&model.mesh.normals)
            .with_indices(&model.mesh.indices)
            .with_texcoords(&model.mesh.texcoords);
        let mesh = BaseMesh { mesh_data };
        let shader = match &material {
            Some((gl_mat,tobj_mat)) => {
                if tobj_mat.ambient.is_some() {
                    Arc::new(new_unlit_color_shader(gl_mat.data.ambient.as_ref().into()))
                } else {
                    NO_SHADER.clone()
                }
            }
            None => NO_SHADER.clone(),
        };
        drawables.push(DrawData {
            mesh: Arc::new(Mutex::new(mesh)),
            shader,
            material: material.map(|(gl_mat,_)| gl_mat)
        })
    }

    Drawable {
        draw_data: drawables,
    }
}

pub fn import_no_mat<P: AsRef<Path> + std::fmt::Debug>(path: P) -> Drawable {
    let (models, _) = load_obj(&path, &tobj::GPU_LOAD_OPTIONS).expect("Failed to load obj file"); //TODO: handle error

    let mut drawables: Vec<DrawData> = vec![];
    for model in models.iter() {
        let mesh_data = MeshData::new(&model.mesh.positions)
            .with_normals(&model.mesh.normals)
            .with_indices(&model.mesh.indices)
            .with_texcoords(&model.mesh.texcoords);
        let mesh = BaseMesh { mesh_data };
        let shader = NO_SHADER.clone();
        drawables.push(DrawData {
            mesh: Arc::new(Mutex::new(mesh)),
            shader,
            material: None,
        })
    }

    Drawable {
        draw_data: drawables,
    }
}
