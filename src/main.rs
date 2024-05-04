mod engine;

use std::f32::consts::PI;

extern crate glfw;

use glfw::{Action, Context, Key, WindowHint};
use std::sync::mpsc::{channel, Receiver};
use std::thread::Builder;
use nalgebra_glm as glm;
use crate::engine::drawable::Drawable;
use crate::engine::drawable::mesh::cube::CubeMesh;
use crate::engine::shader::Shader;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    let mut glfw = glfw::init_no_callbacks().unwrap();

    glfw.window_hint(WindowHint::ContextVersion(3, 3));
    glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(WindowHint::Resizable(false));
    glfw.window_hint(WindowHint::Samples(Some(4))); // Set the number of samples for multi-sampling

    let (mut window, events) = glfw
        .create_window(WIDTH, HEIGHT, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    let render_context = window.render_context();
    let (send, recv) = channel();

    let render_task = Builder::new().name("render task".to_string());
    let render_task_done = render_task.spawn(move || {
        render(render_context, recv);
    });

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }

    // Tell the render task to exit.
    send.send(()).ok().expect("Failed signal to render thread.");

    // Wait for acknowledgement that the rendering was completed.
    let _ = render_task_done;
}

fn render(mut ctx: glfw::PRenderContext, finish: Receiver<()>) {
    ctx.make_current();
    gl::load_with(|symbol| ctx.get_proc_address(symbol) as *const _);

    let shader = Shader::default();
    shader.use_program();

    let global_pos = glm::vec3(0.0, 1.0, 0.0);
    //to mat4
    let model = glm::translation(&global_pos);
    shader.set_mat4("model", &model);

    let eye = glm::vec3(0.0, 0.0, 3.0); // Camera position
    let center = glm::vec3(0.0, 0.0, 0.0); // Camera target
    let up = glm::vec3(0.0, 1.0, 0.0); // Up vector

    // Create a view matrix
    let view = glm::look_at(&eye, &center, &up);

    // Define rotation parameters
    let angle = (PI / 180.0) * 15.0; // Rotation angle in degrees
    let axis = glm::vec3(0.0, 1.0, 0.0); // Rotation axis

    // Create a rotation matrix
    let rotation = glm::rotation(angle, &axis);

    // Apply rotation to the view matrix
    let rotated_view = view * rotation;

    // Set the view matrix in the shader
    shader.set_mat4("view", &rotated_view);

    let fov: f32 = 70.0; // Field of view in degrees
    let aspect_ratio: f32 = WIDTH as f32 / HEIGHT as f32; // Aspect ratio of the window
    let near_clip: f32 = 0.1; // Near clipping plane
    let far_clip: f32 = 100.0; // Far clipping plane

    // Create the perspective projection matrix
    let projection = glm::perspective(aspect_ratio, fov.to_radians(), near_clip, far_clip);

    // Set the projection matrix in the shader
    shader.set_mat4("projection", &projection);


    unsafe {
        gl::Enable(gl::MULTISAMPLE); // Enable multi-sampling
        gl::Enable(gl::BLEND); // Enable blending for better anti-aliasing
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA); // Set blending function
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Enable(gl::DEPTH_TEST);
    }

    let cube = CubeMesh::default();

    let mut viewport = glm::vec4(0.0, 0.0, WIDTH as f32, HEIGHT as f32);

    loop {
        // Check if the rendering should stop.
        if finish.try_recv() == Ok(()) {
            break;
        };

        unsafe {
            gl::Viewport(viewport.x as i32, viewport.y as i32, WIDTH as i32, HEIGHT as i32);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            cube.draw();
            gl::DrawElements(gl::TRIANGLES, 36, gl::UNSIGNED_INT, std::ptr::null());
        }

        ctx.swap_buffers();
    }
    // required on some platforms
    glfw::make_context_current(None);

}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}