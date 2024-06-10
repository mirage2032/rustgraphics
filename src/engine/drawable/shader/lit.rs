use std::sync::Arc;
use lazy_static::lazy_static;
use crate::engine::drawable::shader::Shader;
use crate::result::EngineRenderResult;

pub fn new_basic_shader() -> EngineRenderResult<Shader> {
    Shader::new(
        Some(include_str!("glsl/lit/basic/vertex_shader.glsl")),
        Some(include_str!("glsl/lit/basic/fragment_shader.glsl")),
        None,
    )
}

lazy_static! {
    pub static ref LIT_COLOR_SHADER: Arc<Shader> = Arc::new(new_basic_shader().unwrap());
}