use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::engine::drawable::base::BaseDrawable;
use crate::engine::drawable::material::Material;
use crate::engine::drawable::mesh::{BaseMesh, MeshData};
use crate::engine::drawable::shader::lit::LIT_COLOR_SHADER;
use crate::engine::drawable::shader::Shader;
use crate::engine::drawable::DrawData;
use glengine_mdl::models::FileStruct;

pub fn import(nmdl: &FileStruct) -> BaseDrawable {
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
                .iter()
                .flat_map(|v| vec![v.0, v.1, v.2])
                .collect::<Vec<f32>>(),
        )
        .with_normals(
            &mesh
                .normals
                .iter()
                .flat_map(|v| vec![v.0, v.1, v.2])
                .collect::<Vec<f32>>(),
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
                    .iter()
                    .flat_map(|v| vec![v.0, v.1])
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