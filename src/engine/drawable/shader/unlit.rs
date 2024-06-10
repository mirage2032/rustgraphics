use std::sync::Arc;
use lazy_static::lazy_static;
use crate::engine::drawable::shader::Shader;
use crate::result::EngineRenderResult;

pub fn new_face_shader() -> EngineRenderResult<Shader> {
    Shader::new(
        Some(include_str!("glsl/unlit/face/vertex_shader.glsl")),
        Some(include_str!("glsl/unlit/face/fragment_shader.glsl")),
        Some(include_str!("glsl/unlit/face/geometry_shader.glsl")),
    )
}

pub fn new_quad_shader() -> EngineRenderResult<Shader> {
    Shader::new(
        Some(include_str!("glsl/screen/vertex_shader.glsl")),
        Some(include_str!("glsl/screen/fragment_shader.glsl")),
        None,
    )
}

lazy_static! {
    pub static ref FACE_SHADER: Arc<Shader> = Arc::new(new_face_shader().unwrap());
    pub static ref QUAD_SHADER: Arc<Shader> = Arc::new(new_quad_shader().unwrap());
}
