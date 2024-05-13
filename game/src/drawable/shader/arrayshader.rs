use glengine::engine::drawable::shader::Shader;
use glengine::result::EngineRenderResult;

pub fn build_array_shader() -> EngineRenderResult<Shader> {
    let array_vertex_shader = include_str!("glsl/array_shader/vertex_shader.glsl");
    let array_fragment_shader = include_str!("glsl/array_shader/fragment_shader.glsl");
    Shader::new(Some(array_vertex_shader), Some(array_fragment_shader), None)
}
