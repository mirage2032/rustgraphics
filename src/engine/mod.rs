use std::sync::mpsc::channel;
use std::thread::{Builder};
use glfw::{PWindow, WindowHint, Glfw, GlfwReceiver, WindowEvent};
use crate::{handle_window_event, HEIGHT, render, WIDTH};

pub mod drawable;
pub mod shader;

pub struct Engine {
    glfw: Glfw,
    window: PWindow,
    events: GlfwReceiver<(f64,WindowEvent)>
}

impl Engine {
    pub fn new() -> Self {
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
        Self{window,glfw,events}
    }

    pub fn run(&mut self) {
        let render_context = self.window.render_context();
        let (send, recv) = channel();

        let render_task = Builder::new().name("render task".to_string());
        let render_task_done = render_task.spawn(move || {
            render(render_context, recv);
        });

        while !self.window.should_close() {
            self.glfw.poll_events();
            for (_, event) in glfw::flush_messages(&self.events) {
                handle_window_event(&mut self.window, event);
            }
        }

        // Tell the render task to exit.
        send.send(()).ok().expect("Failed signal to render thread.");

        // Wait for acknowledgement that the rendering was completed.
        let _ = render_task_done;
    }
}