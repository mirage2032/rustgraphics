use crate::engine::drawable::shader::Shader;
use crate::result::EngineRenderResult;
use once_cell::unsync::Lazy;
use std::cell::RefCell;
use std::rc::Rc;

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