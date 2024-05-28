use gl;
use gl::types::GLuint;
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
    pub diffuse_texture: Option<Texture>,
}

impl From<tobj::Material> for Material {
    fn from(material: tobj::Material) -> Self {
        let data = MaterialData {
            ambient: material.ambient.map(|a| a.into()),
            diffuse: material.diffuse.map(|a| a.into()),
            specular: material.specular.map(|a| a.into()),
            shininess: material.shininess.map(|a| a.into()),
        };

        let diffuse_texture = {
            match material.diffuse_texture {
                Some(diffuse_texture) => {
                    let image = Image::load(&diffuse_texture)
                        .expect(format!("Failed to load texture: {}", diffuse_texture).as_str());
                    Some(Texture::from(image))
                }
                None => None,
            }
        };

        Self {
            data,
            diffuse_texture,
        }
    }
}

impl From<russimp::material::Material> for Material {
    fn from(material: russimp::material::Material) -> Self {
        let mut data = MaterialData::default();
        material
            .properties
            .iter()
            .for_each(|prop| match (&*prop.key, &prop.data) {
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
                    data.shininess = Some(val[0] / 250.0);
                }
                // (a,b) => {
                //     println!("Unknown property: {:?} {:?}", a,b);
                // }
                _ => {}
            });
        let diffuse_texture = match material.textures.get(&TextureType::Diffuse) {
            Some(texture) => {
                let image = Image::load(&texture.borrow().filename).expect(
                    format!("Failed to load texture: {}", &texture.borrow().filename).as_str(),
                );
                Some(Texture::from(image))
            }
            None => None,
        };

        Self {
            data,
            diffuse_texture,
        }
    }
}
impl Material {
    pub fn set_uniforms(&self, shader: &Shader) {
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
        // if let Some(texture) = &self.diffuse_texture {
        //     shader.set_texture("material.diffuse_texture", texture.id(), 0);
        // }
    }
}

pub struct Texture {
    id: GLuint,
}

impl Texture {
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
        Self { id: texture }
    }
}
