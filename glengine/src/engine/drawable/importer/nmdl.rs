use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::engine::drawable::base::BaseDrawable;
use crate::engine::drawable::material::{Material, MATERIAL_MAP};
use crate::engine::drawable::mesh::{BaseMesh, MeshData, MESH_MAP};
use crate::engine::drawable::shader::{IncludedShaderType, Shader, ShaderType};
use crate::engine::drawable::DrawData;
use glengine_mdl::models::{FileStruct};
use crate::engine::scene::gameobject::components::collider::ColliderComponent;

pub fn import(path: &str) -> BaseDrawable {
    let nmdl = FileStruct::load(path).expect("Could not load NMDL");
    let mut materials: HashMap<u32,usize> = HashMap::new();
    let mut draw_data: Vec<DrawData> = vec![];
    let mut material_map = MATERIAL_MAP.lock().expect("Could not lock material map");
    nmdl.meshes.iter().for_each(|mesh| {
        let material_id = match materials.get(&mesh.material_index){
            Some(mat) => *mat,
            None => {
                let mat:Material = nmdl.materials.materials[mesh.material_index as usize].clone().into();
                let material_id = material_map.add(mat);
                materials.insert(mesh.material_index, material_id);
                material_id
            }
        };
        let material = material_map.get(material_id).expect("Material not found");

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
        let shader_type = match material.data.ambient{
            Some(_) => ShaderType::Included(IncludedShaderType::LitColor),
            None => ShaderType::Included(IncludedShaderType::Basic),
        };
        let mesh_id = MESH_MAP.with(|mm| mm.borrow_mut().add(Box::new(BaseMesh { mesh_data })));

        let draw = DrawData {
            mesh_handle: mesh_id,
            shader_type,
            material_id: Some(material_id),
        };
        draw_data.push(draw);
    });
    BaseDrawable { draw_data }
}
pub fn import_w_collider(path: &str,scale:f32) -> (BaseDrawable,ColliderComponent) {
    let nmdl = FileStruct::load(path).expect("Could not load NMDL");
    let mut materials: HashMap<u32,usize> = HashMap::new();
    let mut draw_data: Vec<DrawData> = vec![];
    let mut material_map = MATERIAL_MAP.lock().expect("Could not lock material map");
    nmdl.meshes.iter().for_each(|mesh| {
        let material_id = match materials.get(&mesh.material_index){
            Some(mat) => mat.clone(),
            None => {
                let mat:Material = nmdl.materials.materials[mesh.material_index as usize].clone().into();
                let material_id = material_map.add(mat);
                materials.insert(mesh.material_index, material_id);
                material_id
            }
        };
        
        let material = material_map.get(material_id).expect("Material not found");

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
        let shader_type = match material.data.ambient{
            Some(_) => ShaderType::Included(IncludedShaderType::LitColor),
            None => ShaderType::Included(IncludedShaderType::Basic),
        };
        let mesh_id = MESH_MAP.with(|mm| mm.borrow_mut().add(Box::new(BaseMesh { mesh_data })));

        let draw = DrawData {
            mesh_handle: mesh_id,
            shader_type,
            material_id: Some(material_id),
        };
        draw_data.push(draw);
    });
    (BaseDrawable { draw_data },ColliderComponent::hull_from_meshvec(&nmdl.meshes,scale))
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

#[macro_export]
macro_rules! nmdl_import_w_collider {
    ($mdl_path:expr,$scale:expr) =>{{
        use glengine::build_utils::models::convert_name;
        use glengine::engine::drawable::importer::nmdl::import;
        use std::path::PathBuf;
        let location = std::env::current_exe().unwrap().parent().unwrap().join("models").join($mdl_path);
        let nmdl_location = convert_name(&location,&PathBuf::new(),&PathBuf::new());
        import_w_collider(nmdl_location.to_str().unwrap(),$scale)
    }
}}