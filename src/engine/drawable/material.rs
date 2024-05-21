use gl;
use gl::types::GLuint;
use glam::{Vec3,vec3};

use crate::engine::drawable::importer::img::Image;
use crate::engine::drawable::shader::Shader;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MaterialData {
    pub ambient: Vec3,
    pub diffuse: Vec3,
    pub specular: Vec3,
    pub shininess: f32,
}

impl Default for MaterialData {
    fn default() -> Self {
        Self {
            ambient: vec3(1.0, 1.0, 1.0),
            diffuse: vec3(1.0, 1.0, 1.0),
            specular: vec3(1.0, 1.0, 1.0),
            shininess: 32.0,
        }
    }
}

pub struct Material {
    pub data: MaterialData,
    pub diffuse_texture: Option<Texture>,
}

impl From<tobj::Material> for Material {
    fn from(material: tobj::Material) -> Self {
        let ambient: Vec3 = material.ambient.unwrap_or([1.0, 1.0, 1.0]).into();
        let diffuse: Vec3 = material.diffuse.unwrap_or([1.0, 1.0, 1.0]).into();
        let specular: Vec3 = material.specular.unwrap_or([1.0, 1.0, 1.0]).into();
        let shininess = material.shininess.unwrap_or(32.0);
        let data = MaterialData {
            ambient,
            diffuse,
            specular,
            shininess,
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
impl Material {
    pub fn set_uniforms(&self, shader: &Shader) {
        shader.set_vec3("material.ambient", &self.data.ambient);
        shader.set_vec3("material.diffuse", &self.data.diffuse);
        shader.set_vec3("material.specular", &self.data.specular);
        shader.set_float("material.shininess", self.data.shininess);
        if let Some(texture) = &self.diffuse_texture {
            shader.set_texture("material.diffuse_texture", texture.id(), 0);
        }
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
