use gl;

use crate::engine::drawable::shader::Shader;
use crate::result::EngineRenderResult;

pub fn new_unlit_color_shader() -> EngineRenderResult<Shader> {
    Shader::new(
        Some(include_str!("glsl/unlit/basic/vertex_shader.glsl")),
        Some(include_str!("glsl/unlit/color/fragment_shader.glsl")),
        None,
    )
}
pub fn new_lit_color_shader() -> EngineRenderResult<Shader> {
    Shader::new(
        Some(include_str!("glsl/lit/color/vertex_shader.glsl")),
        Some(include_str!("glsl/lit/color/fragment_shader.glsl")),
        Some(include_str!("glsl/lit/color/geometry_shader.glsl")),
    )
}
