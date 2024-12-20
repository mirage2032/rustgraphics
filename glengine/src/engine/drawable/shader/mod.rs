use gl;
use gl::types::GLenum;
use glam::Mat4;
use std::ops::AddAssign;
use std::sync::RwLock;
use crate::result::{EngineRenderResult, ShaderError};

pub mod lit;
pub mod unlit;
pub mod manager;

pub struct Shader {
    id: u32,
    texture_count: RwLock<u32>,
}

impl Shader {
    pub fn new(
        vertex_shader: Option<&str>,
        fragment_shader: Option<&str>,
        geometry_shader: Option<&str>,
    ) -> EngineRenderResult<Shader> {
        // Link shaders
        let shader = Shader {
            id: unsafe { gl::CreateProgram() },
            texture_count: RwLock::new(0),
        };
        unsafe {
            if let Some(vertex_shader_path) = vertex_shader {
                shader.compile_and_attach_shader(vertex_shader_path, gl::VERTEX_SHADER)?;
            }
            if let Some(fragment_shader_path) = fragment_shader {
                shader.compile_and_attach_shader(fragment_shader_path, gl::FRAGMENT_SHADER)?;
            }
            if let Some(geometry_shader_path) = geometry_shader {
                shader.compile_and_attach_shader(geometry_shader_path, gl::GEOMETRY_SHADER)?;
            }
            gl::LinkProgram(shader.id);
            //check error
            let mut success = 0;
            gl::GetProgramiv(shader.id, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut len = 0;
                gl::GetProgramiv(shader.id, gl::INFO_LOG_LENGTH, &mut len);
                let mut buffer = vec![0; len as usize];
                gl::GetProgramInfoLog(
                    shader.id,
                    len,
                    std::ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut i8,
                );
                println!(
                    "Failed to link shader: {}",
                    String::from_utf8(buffer).unwrap()
                );
                return Err(ShaderError::LinkError.into());
            }
        }
        Ok(shader)
    }
    fn compile_shader(source: &str, shader_type: GLenum) -> EngineRenderResult<u32> {
        let id = unsafe { gl::CreateShader(shader_type) };
        unsafe {
            let c_str = std::ffi::CString::new(source.as_bytes()).unwrap();
            gl::ShaderSource(id, 1, &c_str.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }
        let mut success = 0;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            let mut len = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }
            let mut buffer = vec![0; len as usize];
            unsafe {
                gl::GetShaderInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut i8,
                );
            }
            let error = String::from_utf8(buffer).unwrap();
            eprintln!("Failed to compile shader: {}", error);
            return Err(ShaderError::CompileError.into());
        }

        Ok(id)
    }

    fn compile_and_attach_shader(
        &self,
        source: &str,
        shader_type: GLenum,
    ) -> EngineRenderResult<()> {
        let shader = Self::compile_shader(source, shader_type)?;
        unsafe {
            gl::AttachShader(self.id, shader);
        }
        Ok(())
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
    
    pub fn unbind(){
        unsafe {
            gl::UseProgram(0);
        }
    }

    pub fn set_texture(&self, name: &str, texture: u32, index: u32, texture_type: GLenum) {
        unsafe {
            let name_cstring = std::ffi::CString::new(name).unwrap();
            let location = gl::GetUniformLocation(self.id, name_cstring.as_ptr());
            gl::ActiveTexture(gl::TEXTURE0 + index);
            gl::BindTexture(texture_type, texture);
            gl::Uniform1i(location, index as i32);
        }
    }

    pub fn add_texture(&self, name: &str, texture: u32, texture_type: GLenum) {
        if let Ok(mut count) = self.texture_count.try_write(){
            self.set_texture(name, texture, *count, texture_type);
            count.add_assign(1);
        }
    }
    
    pub fn reset_texture_count(&self) {
        if let Ok(mut count) = self.texture_count.try_write(){
            *count = 0;
        }
    }

    pub fn set_float(&self, name: &str, value: f32) {
        unsafe {
            let name_cstring = std::ffi::CString::new(name).unwrap();
            let location = gl::GetUniformLocation(self.id, name_cstring.as_ptr());
            gl::Uniform1f(location, value);
        }
    }

    pub fn set_vec3(&self, name: &str, vec: &glam::Vec3) {
        unsafe {
            let name_cstring = std::ffi::CString::new(name).unwrap();
            let location = gl::GetUniformLocation(self.id, name_cstring.as_ptr());
            gl::Uniform3fv(location, 1, vec.as_ref().as_ptr());
        }
    }

    pub fn set_mat4(&self, name: &str, mat: &Mat4) {
        unsafe {
            let name_cstring = std::ffi::CString::new(name).unwrap();
            let location = gl::GetUniformLocation(self.id, name_cstring.as_ptr());
            gl::UniformMatrix4fv(location, 1, gl::FALSE, mat.as_ref().as_ptr());
        }
    }

    pub fn set_uniform_block(&self, name: &str, binding: u32) {
        unsafe {
            let name_cstring = std::ffi::CString::new(name).unwrap();
            let index = gl::GetUniformBlockIndex(self.id, name_cstring.as_ptr());
            gl::UniformBlockBinding(self.id, index, binding);
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

impl Default for Shader {
    fn default() -> Self {
        unlit::new_face_shader().unwrap()
    }
}