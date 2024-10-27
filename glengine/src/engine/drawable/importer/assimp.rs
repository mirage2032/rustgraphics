use russimp::scene::PostProcess;
use russimp::scene::Scene;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::engine::drawable::base::BaseDrawable;
use crate::engine::drawable::material::{Material, MATERIAL_MAP};
use crate::engine::drawable::mesh::{BaseMesh, MeshData, MESH_MAP};
use crate::engine::drawable::shader::{IncludedShaderHandle};
use crate::engine::drawable::DrawData;

pub fn import(path: &str) -> BaseDrawable {
    let scene = Scene::from_file(
        path,
        vec![
            PostProcess::CalculateTangentSpace,
            PostProcess::OptimizeMeshes,
            PostProcess::OptimizeGraph,
            PostProcess::Triangulate,
            PostProcess::JoinIdenticalVertices,
            PostProcess::GenerateNormals,
            PostProcess::SortByPrimitiveType,
        ],
    )
    .expect("Failed to load obj file");

    let mut materials: HashMap<u32,usize> = HashMap::new();
    let mut draw_data: Vec<DrawData> = vec![];
    let mut material_map = MATERIAL_MAP.lock().expect("Could not lock material map");
    scene.meshes.iter().for_each(|mesh| {
        let material_id = match materials.get(&mesh.material_index){
            Some(mat) => mat.clone(),
            None => {
                let mat:Material = scene.materials[mesh.material_index as usize].clone().into();
                let material_id = material_map.add(mat);
                materials.insert(mesh.material_index, material_id);
                material_id
            }
        };
        
        let material = material_map.get(material_id).expect("Material not found").clone();
        
        let mut mesh_data = MeshData::new(
            &mesh
                .vertices
                .iter()
                .flat_map(|v| vec![v.x, v.y, v.z])
                .collect::<Vec<f32>>(),
        )
        .with_normals(
            &mesh
                .normals
                .iter()
                .flat_map(|v| vec![v.x, v.y, v.z])
                .collect::<Vec<f32>>(),
        )
        .with_indices(
            &mesh
                .faces
                .iter()
                .flat_map(|face| face.0.clone())
                .collect::<Vec<u32>>(),
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
        if let Some(Some(tex_coords)) = mesh.texture_coords.get(0){
            mesh_data = mesh_data.with_texcoords(
                &tex_coords
                    .iter()
                    .flat_map(|v| vec![v.x, v.y])
                    .collect::<Vec<f32>>(),
            );
        }
        let shader_handle = match material.data.ambient{
            Some(_) => IncludedShaderHandle::LitColor.into(),
            None => IncludedShaderHandle::Basic.into(),
        };
        let mesh_handle = MESH_MAP.with(|mm| mm.borrow_mut().add(Box::new(BaseMesh{mesh_data})));
        
        let draw = DrawData {
            mesh_handle,
            shader_handle,
            material_id: Some(material_id),
        };
        draw_data.push(draw);
    });

    BaseDrawable { draw_data }
}