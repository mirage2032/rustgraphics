use std::f32::consts::PI;
use std::sync::mpsc::{channel, Receiver};
use std::thread::Builder;

use glfw::{Action, Context, Glfw, GlfwReceiver, Key, PWindow, WindowEvent, WindowHint};
use nalgebra_glm as glm;

use crate::{HEIGHT, WIDTH};
use crate::engine::drawable::cube::DrawCube;
use crate::engine::drawable::Drawable;

pub mod drawable;
pub mod shader;
pub mod gameobject;
pub mod transform;
mod scene;
mod camera;

pub struct Engine {
    glfw: Glfw,
    window: PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
}

impl Engine {
    pub fn new() -> Self {
        let mut glfw = glfw::init_no_callbacks().unwrap();

        glfw.window_hint(WindowHint::ContextVersion(3, 3));
        glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(WindowHint::Resizable(false));
        glfw.window_hint(WindowHint::Samples(Some(4))); // Set the number of samples for multi-sampling

        let (mut window, events) = glfw
            .create_window(*WIDTH, *HEIGHT, "Hello this is window", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_key_polling(true);
        window.glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
        Self { window, glfw, events }
    }

    pub fn run(&mut self) {
        let render_context = self.window.render_context();
        let (send, recv) = channel();

        let render_task = Builder::new().name("render task".to_string());
        let render_task_done = render_task.spawn(move || {
            Self::render(render_context, recv);
        });

        while !self.window.should_close() {
            self.glfw.poll_events();
            for (_, event) in glfw::flush_messages(&self.events) {
                Self::handle_window_event(&mut self.window, event);
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

        //to mat4
        let mut model = {
            let global_pos = glm::vec3(0.0, 1.0, 0.0);
            glm::translation(&global_pos)
        };

        // Apply rotation to the view matrix
        let rotated_view = {
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
            view * rotation
        };

        unsafe {
            gl::Enable(gl::MULTISAMPLE); // Enable multi-sampling
            gl::Enable(gl::BLEND); // Enable blending for better anti-aliasing
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA); // Set blending function
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Enable(gl::DEPTH_TEST);
        }

        let cube = DrawCube::default();

        let mut viewport = glm::vec4(0.0, 0.0, *WIDTH as f32, *HEIGHT as f32);

        loop {
            let rotate_offset = glm::vec3(0.45,0.7,0.2);
            let translate_offset = glm::vec3(0.0,-0.005,0.0);
            model = glm::rotate(&model, 0.03, &rotate_offset);
            let translate_global = glm::translation(&translate_offset);
            model = translate_global * model;

            // Check if the rendering should stop.
            if finish.try_recv() == Ok(()) {
                break;
            };

            unsafe {
                gl::Viewport(viewport.x as i32, viewport.y as i32, *WIDTH as i32, *HEIGHT as i32);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                cube.draw(&model, &rotated_view);
                gl::DrawElements(gl::TRIANGLES, 36, gl::UNSIGNED_INT, std::ptr::null());
            }

            ctx.swap_buffers();
        }
        // required on some platforms
        glfw::make_context_current(None);
    }

    fn handle_window_event(window: &mut glfw::Window, event: WindowEvent) {
        match event {
            WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }
    }
}