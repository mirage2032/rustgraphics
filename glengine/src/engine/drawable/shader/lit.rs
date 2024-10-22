use crate::engine::drawable::shader::Shader;
use crate::result::EngineRenderResult;
use once_cell::unsync::Lazy;
use std::cell::RefCell;
use std::rc::Rc;

pub fn new_basic_shader() -> EngineRenderResult<Shader> {
    Shader::new(
        Some(include_str!("glsl/lit/basic/vertex_shader.glsl")),
        Some(include_str!("glsl/lit/basic/fragment_shader.glsl")),
        None,
    )
}

pub const LIT_COLOR_SHADER: Lazy<Rc<RefCell<Shader>>> = Lazy::new(|| {
    Rc::new(RefCell::new(new_basic_shader().unwrap()))
});