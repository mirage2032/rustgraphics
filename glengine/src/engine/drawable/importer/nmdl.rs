use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::engine::drawable::base::BaseDrawable;
use crate::engine::drawable::material::Material;
use crate::engine::drawable::mesh::{BaseMesh, MeshData};
use crate::engine::drawable::shader::lit::LIT_COLOR_SHADER;
use crate::engine::drawable::shader::Shader;
use crate::engine::drawable::DrawData;
use glengine_mdl::models::{FileStruct, EXTENSION};
use crate::build_utils::models::convert_name;

pub fn import(path: &str) -> BaseDrawable {
    let nmdl = FileStruct::load(path).expect("Could not load NMDL");
    let mut materials: HashMap<u32,Rc<Material>> = HashMap::new();
    let mut draw_data: Vec<DrawData> = vec![];
    nmdl.meshes.iter().for_each(|mesh| {
        let material = match materials.get(&mesh.material_index){
            Some(mat) => mat.clone(),
            None => {
                let mat:Material = nmdl.materials.materials[mesh.material_index as usize].clone().into();
                let mat_arc = Rc::new(mat);
                materials.insert(mesh.material_index, mat_arc.clone());
                mat_arc
            }
        };
        
        let mut mesh_data = MeshData::new(
            &mesh
                .vertices
        )
        .with_normals(
            &mesh
                .normals
        )
        .with_indices(
            &mesh.indices
        );
        if mesh.normals.len() == 0 {
            mesh_data = mesh_data.with_normals(
                &mesh
                    .vertices
                    .iter()
                    .flat_map(|_| vec![0.0, 0.0, 0.0])
                    .collect::<Vec<f32>>(),
            );
        }
        if let Some(tex_coords) = &mesh.texture_coords{
            mesh_data = mesh_data.with_texcoords(
                &tex_coords
            );
        }
        let shader = match material.data.ambient{
            Some(_) => LIT_COLOR_SHADER.clone(),
            None => Rc::new(RefCell::new(Shader::default())),
        };
        
        let draw = DrawData {
            mesh: Rc::new(RefCell::new(BaseMesh { mesh_data })),
            shader,
            material: Some(material),
        };
        draw_data.push(draw);
    });
    BaseDrawable { draw_data }
}
#[macro_export]
macro_rules! nmdl_import {
    ($mdl_path:expr) =>{{
        use glengine::build_utils::models::convert_name;
        use glengine::engine::drawable::importer::nmdl::import;
        use std::path::PathBuf;
        let location = std::env::current_exe().unwrap().parent().unwrap().join("models").join($mdl_path);
        let nmdl_location = convert_name(&location,&PathBuf::new(),&PathBuf::new());
        import(nmdl_location.to_str().unwrap())
    }
}}