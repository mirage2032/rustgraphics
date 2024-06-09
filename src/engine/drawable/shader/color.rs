use crate::engine::drawable::shader::Shader;
use crate::result::EngineRenderResult;

pub fn new_lit_color_shader() -> EngineRenderResult<Shader> {
    Shader::new(
        Some(include_str!("glsl/lit/basic/vertex_shader.glsl")),
        Some(include_str!("glsl/lit/basic/fragment_shader.glsl")),
        None,
    )
}