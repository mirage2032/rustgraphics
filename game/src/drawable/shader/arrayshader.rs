use glengine::engine::shader::{DEFAULT_FRAGMENT_SHADER, Shader};
use glengine::error::EngineResult;

pub fn build_array_shader() -> EngineResult<Shader> {
    let array_vertex_shader = include_str!("glsl/vertex/array_shader.glsl");
    Shader::new(Some(array_vertex_shader), Some(DEFAULT_FRAGMENT_SHADER),None)
}
