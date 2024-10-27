use std::collections::HashMap;

use crate::engine::drawable::base::BaseDrawable;
use crate::engine::drawable::material::{Material, manager::MaterialHandle};
use crate::engine::drawable::mesh::{BaseMesh, MeshData};
use crate::engine::drawable::shader::manager::{IncludedShaderHandle, ShaderHandle};
use crate::engine::drawable::DrawData;
use glengine_mdl::models::{FileStruct, MeshStruct};
use crate::engine::drawable::manager::DRAWABLE_MANAGER;
use crate::engine::scene::gameobject::components::collider::ColliderComponent;

fn import_w_mesh(path: &str) -> (BaseDrawable,Vec<MeshStruct>) {
    let nmdl = FileStruct::load(path).expect("Could not load NMDL");
    let mut materials: HashMap<u32,(MaterialHandle,ShaderHandle)> = HashMap::new();
    let mut draw_data: Vec<DrawData> = vec![];
    nmdl.meshes.iter().for_each(|mesh| {
        let (material_handle,shader_handle)= DRAWABLE_MANAGER.with(|mut dm| {
            match materials.get(&mesh.material_index) {
                Some(mat) => mat.clone(),
                None => {
                    let mat: Material = nmdl.materials.materials[mesh.material_index as usize].clone().into();
                    let shader_handle:ShaderHandle = match mat.data.ambient{
                        Some(_) => IncludedShaderHandle::LitColor.into(),
                        None => IncludedShaderHandle::Basic.into(),
                    };
                    let material_handle = dm.borrow_mut().material.add(mat);
                    materials.insert(mesh.material_index, (material_handle.clone(),shader_handle.clone()));
                    (material_handle,shader_handle)
                }
            }
        });
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
        let mesh_handle = DRAWABLE_MANAGER.with(|dm| dm.borrow_mut().mesh.add(Box::new(BaseMesh { mesh_data })));

        let draw = DrawData {
            mesh_handle,
            shader_handle,
            material_handle: Some(material_handle),
        };
        draw_data.push(draw);
    });
    (BaseDrawable { draw_data },nmdl.meshes)
}
pub fn import(path: &str) -> BaseDrawable {
    import_w_mesh(path).0
}
pub fn import_w_collider(path: &str,scale:f32) -> (BaseDrawable,ColliderComponent) {
    let (draw_data,meshes) = import_w_mesh(path);
    (draw_data,ColliderComponent::hull_from_meshvec(&meshes,scale))
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