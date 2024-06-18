use std::collections::HashMap;
use std::sync::Arc;

use gl;
use gl::types::{GLenum, GLuint};
use glam::{vec3, Vec3};
use russimp::material::{PropertyTypeInfo, TextureType};

use crate::engine::drawable::importer::img::Image;
use crate::engine::drawable::shader::Shader;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MaterialData {
    pub ambient: Option<Vec3>,
    pub diffuse: Option<Vec3>,
    pub specular: Option<Vec3>,
    pub shininess: Option<f32>,
}

impl Default for MaterialData {
    fn default() -> Self {
        Self {
            ambient: None,
            diffuse: None,
            specular: None,
            shininess: None,
        }
    }
}

pub struct Material {
    pub data: MaterialData,
    pub textures: HashMap<&'static str, Texture>,
}

impl From<russimp::material::Material> for Material {
    fn from(material: russimp::material::Material) -> Self {
        let mut data = MaterialData::default();
        let mut textures = HashMap::new();

        material
            .properties
            .iter()
            .for_each(|prop| match (&*prop.key, &prop.data) {
                // ("?mat.name", &PropertyTypeInfo::String(ref name)) => {
                //     println!("Material name: {}", name);
                // }
                ("$clr.ambient", &PropertyTypeInfo::FloatArray(ref color)) => {
                    data.ambient = Some(vec3(color[0], color[1], color[2]));
                }
                ("$clr.diffuse", &PropertyTypeInfo::FloatArray(ref color)) => {
                    data.diffuse = Some(vec3(color[0], color[1], color[2]));
                }
                ("$clr.specular", &PropertyTypeInfo::FloatArray(ref color)) => {
                    data.specular = Some(vec3(color[0], color[1], color[2]));
                }
                ("$mat.shininess", &PropertyTypeInfo::FloatArray(ref val)) => {
                    let val = val[0];
                    data.shininess = Some((val / 1000.0)*128.0);
                    // println!("Shininess: {:?}", data.shininess);
                }
                // (a,b) => {
                //     println!("Unknown property: {:?} {:?}", a,b);
                // }
                _ => {}
            });
        if let Some(diffuse_texture) = material.textures.get(&TextureType::Diffuse) {
            let image = Image::load(&diffuse_texture.borrow().filename).expect(
                format!(
                    "Failed to load texture: {}",
                    &diffuse_texture.borrow().filename
                )
                .as_str(),
            );
            textures.insert("diffuse_texture", image.into());
        };

        Self { data, textures }
    }
}
impl Material {
    pub fn set_uniforms(&self, shader: &mut Shader) {
        if let Some(ambient) = self.data.ambient {
            shader.set_vec3("material.ambient", &ambient);
        }
        if let Some(diffuse) = self.data.diffuse {
            shader.set_vec3("material.diffuse", &diffuse);
        }
        if let Some(specular) = self.data.specular {
            shader.set_vec3("material.specular", &specular);
        }
        if let Some(shininess) = self.data.shininess {
            shader.set_float("material.shininess", shininess);
        }

        for (name, texture) in self.textures.iter() {
            shader.add_texture(name, texture.id(), texture.texture_type());
        }
    }
}

pub struct Texture {
    pub id: GLuint,
    pub texture_type: GLenum,
}

impl Texture {
    pub fn new(id: GLuint, texture_type: GLenum) -> Self {
        Self { id, texture_type }
    }
    pub fn texture_type(&self) -> GLenum {
        self.texture_type
    }
    pub fn id(&self) -> GLuint {
        self.id
    }
}
impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}

impl From<Image> for Texture {
    fn from(image: Image) -> Self {
        let mut texture = 0;
        unsafe {
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                image.width as i32,
                image.height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                image.data.as_ptr() as *const std::ffi::c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        Self { id: texture, texture_type: gl::TEXTURE_2D}
    }
}
