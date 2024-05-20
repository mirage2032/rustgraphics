use gl;

use crate::engine::drawable::shader::Shader;

pub fn new_unlit_color_shader(color: &[f32; 3]) -> Shader {
    let shader = Shader::new(
        Some(include_str!("glsl/unlit/basic/vertex_shader.glsl")),
        Some(include_str!("glsl/unlit/color/fragment_shader.glsl")),
        None,
    )
    .expect("Failed to create color shader");
    shader.use_program();
    shader.set_vec3(
        "u_diff_color",
        &glam::Vec3::new(color[0], color[1], color[2]),
    );
    unsafe {
        gl::UseProgram(0);
    }
    shader
}
pub fn new_lit_color_shader(color: &[f32; 3]) -> Shader {
    let shader = Shader::new(
        Some(include_str!("glsl/lit/color/vertex_shader.glsl")),
        Some(include_str!("glsl/lit/color/fragment_shader.glsl")),
        Some(include_str!("glsl/lit/color/geometry_shader.glsl")),
    )
    .expect("Failed to create color shader");
    shader.use_program();
    shader.set_float("light.ambient.intensity", 0.1);
    shader.set_vec3("light.ambient.color", &glam::Vec3::new(1.0, 1.0, 1.0));
    shader.set_vec3("light.diffuse.direction", &glam::Vec3::new(0.0, -1.0, 0.0));

    shader.set_vec3(
        "u_diff_color",
        &glam::Vec3::new(color[0], color[1], color[2]),
    );
    unsafe {
        gl::UseProgram(0);
    }
    shader
}
