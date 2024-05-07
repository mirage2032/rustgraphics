use gl;
use gl::types::GLenum;
use glam::Mat4;

pub struct Shader {
    id: u32,
}

impl Shader {
    fn compile_shader(source: &str, shader_type: GLenum) -> u32 {
        let id = unsafe { gl::CreateShader(shader_type) };
        unsafe {
            let c_str = std::ffi::CString::new(source.as_bytes()).unwrap();
            gl::ShaderSource(id, 1, &c_str.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }
        id
    }
    pub fn new(vertex_shader_source: &str, fragment_shader_source: &str) -> Self {
        let vertex_shader = Self::compile_shader(&vertex_shader_source, gl::VERTEX_SHADER);

        let fragment_shader = Self::compile_shader(&fragment_shader_source, gl::FRAGMENT_SHADER);

        // Link shaders
        let id = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(id, vertex_shader);
            gl::AttachShader(id, fragment_shader);
            gl::LinkProgram(id);
            gl::UseProgram(id);
        }
        Shader { id }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn set_mat4(&self, name: &str, mat: &Mat4) {
        unsafe {
            self.use_program();
            let name_cstring = std::ffi::CString::new(name).unwrap();
            let location = gl::GetUniformLocation(self.id, name_cstring.as_ptr());
            gl::UniformMatrix4fv(location, 1, gl::FALSE, mat.as_ref().as_ptr());
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
        Shader::new(
            include_str!("vertex_shader.glsl"),
            include_str!("fragment_shader.glsl"))
    }
}