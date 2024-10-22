use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::rc::Rc;
use bincode::{error, Decode, Encode};
use image::GenericImageView;
use russimp::material::{DataContent, Material, PropertyTypeInfo, Texture, TextureType};
use russimp::mesh::Mesh;
use russimp::scene::{PostProcess, Scene};

pub const EXTENSION: &str = "nmdl";
pub const MAGIC: &str = "NMDL";
pub const VERSION: u32 = 1;

#[derive(Debug, Encode, Decode,Clone)]
pub struct Header {
    pub magic: String,
    pub version: u32,
}

impl Header {
    fn new() -> Self {
        Header {
            magic: MAGIC.to_string(),
            version: VERSION,
        }
    }
    fn validate(&self) -> bool {
        self.magic == MAGIC
    }
}

#[derive(Debug, Encode, Decode,Clone)]
pub struct MeshStruct {
    pub name: String,
    pub material_index: u32,
    pub vertices: Vec<(f32, f32, f32)>,
    pub normals: Vec<(f32, f32, f32)>,
    pub indices: Vec<u32>,
    pub texture_coords: Option<Vec<(f32, f32)>>,
}

impl MeshStruct{
    pub fn from_assimp_mesh(mesh:&Mesh) -> Self{
        let texture_coords = {
            if mesh.texture_coords.is_empty() {
                None
            }
            else if let Some(vec_vec32d) = &mesh.texture_coords[0]{
                Some(vec_vec32d.iter().map(|v| (v.x,v.y)).collect())
            }
            else{
                None
            }
        };
        MeshStruct{
            name: mesh.name.clone(),
            material_index: mesh.material_index,
            vertices: mesh.vertices.iter().map(|v| (v.x,v.y,v.z)).collect(),
            normals: mesh.normals.iter().map(|v| (v.x,v.y,v.z)).collect(),
            indices: mesh.faces.iter().flat_map(|f| f.0.clone()).collect(),
            texture_coords
        }
    }
}



#[derive(Debug, Encode, Decode,Clone)]
pub struct MaterialStruct {
    pub texture: TexturesStruct,
    pub ambient: Option<(f32, f32, f32)>,
    pub diffuse: Option<(f32, f32, f32)>,
    pub specular: Option<(f32, f32, f32)>,
    pub shininess: Option<f32>,
}

#[derive(Debug, Encode, Decode,Clone)]
pub struct MaterialsStruct{
    pub materials: Vec<MaterialStruct>,
    pub textures: HashMap<String,TextureStruct>,
}

impl MaterialStruct{
    fn from_assimp_material(material:&Material) -> (Self,Vec<Rc<RefCell<Texture>>>){
        let mut texture_filenames = vec![];
        let mut material_struct = MaterialStruct{
            texture: TexturesStruct{
                diffuse: None,
                specular: None,
                normal: None,
            },
            ambient: None,
            diffuse: None,
            specular: None,
            shininess: None,
        };
        material
            .properties
            .iter()
            .for_each(|prop| match (&*prop.key, &prop.data) {
                // ("?mat.name", &PropertyTypeInfo::String(ref name)) => {
                //     println!("Material name: {}", name);
                // }
                ("$clr.ambient", &PropertyTypeInfo::FloatArray(ref color)) => {
                    material_struct.ambient = Some((color[0], color[1], color[2]));
                }
                ("$clr.diffuse", &PropertyTypeInfo::FloatArray(ref color)) => {
                    material_struct.diffuse = Some((color[0], color[1], color[2]));
                }
                ("$clr.specular", &PropertyTypeInfo::FloatArray(ref color)) => {
                    material_struct.specular = Some((color[0], color[1], color[2]));
                }
                ("$mat.shininess", &PropertyTypeInfo::FloatArray(ref val)) => {
                    let val = val[0];
                    material_struct.shininess = Some((val / 1000.0)*128.0);
                    // println!("Shininess: {:?}", data.shininess);
                }
                // (a,b) => {
                //     println!("Unknown property: {:?} {:?}", a,b);
                // }
                _ => {}
            });
        if let Some(diffuse_texture) = material.textures.get(&TextureType::Diffuse) {
            material_struct.texture.diffuse = Some(diffuse_texture.borrow().filename.clone());
            texture_filenames.push(diffuse_texture.clone());
        }
        if let Some(specular_texture) = material.textures.get(&TextureType::Specular) {
            material_struct.texture.specular = Some(specular_texture.borrow().filename.clone());
            texture_filenames.push(specular_texture.clone());
        }
        if let Some(normal_texture) = material.textures.get(&TextureType::Normals) {
            material_struct.texture.normal = Some(normal_texture.borrow().filename.clone());
            texture_filenames.push(normal_texture.clone());
        }
        (material_struct,texture_filenames)
    }
}

#[derive(Debug, Encode, Decode,Clone)]
pub struct TextureStruct {
    pub name: String,
    pub height: u32,
    pub width: u32,
    pub data: Vec<(u8, u8, u8, u8)>,
}

#[derive(Debug, Encode, Decode,Clone)]
pub struct TexturesStruct {
    pub diffuse: Option<String>,
    pub specular: Option<String>,
    pub normal: Option<String>,
}

impl TextureStruct{
    pub fn from_assimp_texture(texture:&Texture) -> Self{
        let width = texture.width;
        let height = texture.height;
        // let texture_data = match texture.data{
        //     DataContent::Texel(data) => {
        //         let data:Vec<(u8,u8,u8,u8)> = data.iter().map(|c| (c.r,c.g,c.b,c.a)).collect();
        //         TextureStruct{
        //             name: texture.filename.clone(),
        //             height,
        //             width,
        //             data,
        //         }
        //     }
        //     DataContent::Bytes(data) => {
        //         let data:Vec<(u8,u8,u8,u8)> = data.chunks(4).map(|c| (c[0],c[1],c[2],c[3])).collect();
        //         TextureStruct{
        //             name: texture.filename.clone(),
        //             height,
        //             width,
        //             data,
        //         }
        //     }
        // };
        let img = image::open(&texture.filename).expect("Failed to load texture");
        let data = img.into_rgba8().into_raw();
        let data = data.chunks(4).map(|c| (c[0],c[1],c[2],c[3])).collect();
        TextureStruct{
            name: texture.filename.clone(),
            height,
            width,
            data,
        }
    }
}

#[derive(Debug, Encode, Decode,Clone)]
pub struct FileStruct {
    pub magic: Header,
    pub meshes: Vec<MeshStruct>,
    pub materials: MaterialsStruct,
}

impl FileStruct{
    pub fn import(path:&str)->FileStruct{
        println!("Importing file: {}",path);
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
        let meshes = vec![];
        let materials = MaterialsStruct{
            materials: vec![],
            textures: HashMap::new(),
        };
        // let meshes = scene.meshes.iter().map(|mesh| MeshStruct::from_assimp_mesh(mesh)).collect();
        // let mut materials: MaterialsStruct = MaterialsStruct{
        //     materials: Vec::new(),
        //     textures: HashMap::new(),
        // };
        // scene.materials.iter().for_each(|material| {
        //     let (material_struct, texture) = MaterialStruct::from_assimp_material(material);
        //     materials.materials.push(material_struct);
        //     for texture in texture{
        //         let tex_struct = TextureStruct::from_assimp_texture(&*texture.borrow());
        //         materials.textures.insert(texture.borrow().filename.clone(), tex_struct);
        //     }
        // });
        FileStruct{
            magic: Header::new(),
            meshes,
            materials,
        }
    }
    pub fn save(&self,path:&str) ->Result<(),Box<dyn Error>>{
        let path = Path::new(path).with_extension(EXTENSION);
        let mut file = File::create(path)?;
        let res = bincode::encode_into_std_write(&self,&mut file, bincode::config::standard())?;
        Ok(())
    }

    pub fn load(path:&str) ->Result<Self,Box<dyn Error>>{
        let mut file = File::open(path)?;
        let data = bincode::decode_from_std_read(&mut file, bincode::config::standard())?;
        Ok(data)
    }
}


fn main() {}