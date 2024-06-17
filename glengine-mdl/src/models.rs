use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

use russimp::scene::{PostProcess, Scene};
use serde::{Deserialize, Serialize};

static EXTENSION: &str = "nmdl";
static MAGIC: &str = "NMDL";
static VERSION: u32 = 1;

#[derive(Debug, Serialize, Deserialize)]
struct Header {
    magic: String,
    version: u32,
}

impl Header {
    fn new() -> Self {
        Header {
            magic: MAGIC.to_string(),
            version: VERSION,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct MeshStruct {
    name: String,
    material_index: u32,
    vertices: Vec<(f32, f32, f32)>,
    normals: Vec<(f32, f32, f32)>,
    indices: Vec<u32>,
    texcoords: Option<Vec<(f32, f32)>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MaterialStruct {
    name: String,
    texture_index: Option<u32>,
    ambient: (f32, f32, f32),
    diffuse: (f32, f32, f32),
    specular: (f32, f32, f32),
    shininess: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct TextureStruct {
    name: String,
    data: Vec<Vec<(u8, u8, u8, u8)>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FileStruct {
    magic: Header,
    meshes: Vec<MeshStruct>,
    materials: HashMap<u32, MaterialStruct>,
    textures: HashMap<u32, TextureStruct>,
}

pub fn convert(input: &str, output: &str) -> Result<(), String> {
    let mut meshes: Vec<MeshStruct> = vec![];
    let mut materials: Vec<MaterialStruct> = vec![];
    let mut textures: Vec<TextureStruct> = vec![];
    let scene = Scene::from_file(
        input,
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
        .expect("Failed to load file");

    scene.meshes.iter().for_each(|mesh| {
        let name = mesh.name.clone();
        let vertices = mesh.vertices.iter().map(|v| (v.x, v.y, v.z)).collect();
        let normals = mesh.normals.iter().map(|v| (v.x, v.y, v.z)).collect();
        let indices = mesh.faces.iter().flat_map(|face| face.0.clone()).collect();
        let texcoords = match mesh.texture_coords.get(0) {
            Some(Some(tex_coords)) => Some(tex_coords.iter().map(|v| (v.x, v.y)).collect()),
            _ => None,
        };
        
        let mesh_struct = MeshStruct {
            name,
            material_index: mesh.material_index,
            vertices,
            normals,
            indices,
            texcoords,
        };
        meshes.push(mesh_struct);
    });


    let output_file = File::create(output).expect("Could not create file");
    let mut writer = std::io::BufWriter::new(output_file);
    Ok(())
}

fn main() {}