use russimp::scene::PostProcess;
use russimp::scene::Scene;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::engine::drawable::base::BaseDrawable;
use crate::engine::drawable::material::Material;
use crate::engine::drawable::mesh::{BaseMesh, MeshData};
use crate::engine::drawable::shader::lit::LIT_COLOR_SHADER;
use crate::engine::drawable::shader::Shader;
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
            PostProcess::PreTransformVertices,
            PostProcess::GenerateNormals,
            PostProcess::SortByPrimitiveType,
        ],
    )
    .expect("Failed to load obj file");

    let mut materials: HashMap<u32,Rc<Material>> = HashMap::new();
    let mut draw_data: Vec<DrawData> = vec![];
    scene.meshes.iter().for_each(|mesh| {
        let material = match materials.get(&mesh.material_index){
            Some(mat) => mat.clone(),
            None => {
                let mat:Material = scene.materials[mesh.material_index as usize].clone().into();
                let mat_arc = Rc::new(mat);
                materials.insert(mesh.material_index, mat_arc.clone());
                mat_arc
            }
        };
        
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