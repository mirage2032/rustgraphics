use glengine::engine::shader::{DEFAULT_FRAGMENT_SHADER, Shader};

pub fn build_array_shader() -> Shader {
    let array_vertex_shader = include_str!("glsl/vertex/array_shader.glsl");
    Shader::new(array_vertex_shader, DEFAULT_FRAGMENT_SHADER)
}
