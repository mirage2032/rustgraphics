use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender};
use std::thread::{Builder, sleep};

use glam::vec4;
use glfw::{Action, Context, Glfw, GlfwReceiver, Key, PRenderContext, PWindow, WindowEvent, WindowHint};

use crate::engine::config::STATIC_DATA;
use crate::engine::fpscounter::FpsCounter;
use crate::engine::scene::Scene;

pub mod drawable;
pub mod shader;
pub mod gameobject;
pub mod transform;
pub mod scene;
pub mod camera;
pub mod fpscounter;
pub mod config;

pub struct GameData {
    pub scene: Option<Box<dyn Scene>>,
}

pub struct Engine {
    window: PWindow,
    game: Arc<Mutex<GameData>>,
    events: GlfwReceiver<(f64, WindowEvent)>,
    glfw: Glfw,
}

impl Engine {
    pub fn new() -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
        glfw.window_hint(WindowHint::ContextVersion(3, 3));
        glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(WindowHint::Resizable(false));
        glfw.window_hint(WindowHint::Samples(Some(4))); // Set the number of samples for multi-sampling

        let resolution = STATIC_DATA.read().expect("Failed to read config").config().get_resolution();
        let (mut window, events) = glfw
            .create_window(resolution.0, resolution.1, "Hello this is window", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.make_current();
        window.set_key_polling(true);
        window.glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
        glfw.make_context_current(None);

        let game = Arc::new(Mutex::new(
            GameData {
                scene: None,
            }));

        Self { window, game, events, glfw }
    }

    pub fn from_game(game: GameData) -> Self {
        let mut engine = Self::new();
        engine.game = Arc::new(Mutex::new(game));
        engine
    }

    pub fn run(&mut self) {
        let game = self.game.clone();
        let ctx = self.window.render_context();

        let render_task = Builder::new().name("render task".to_string());
        let (send, recv) = channel();
        let render_task_done = render_task.spawn(move || {
            Self::render_task(ctx, game, send);
        });

        let game = self.game.clone();
        let step_task = Builder::new().name("step task".to_string());
        let step_task_done = step_task.spawn(move || {
            Self::step_task(game);
        });

        while !self.window.should_close() {
            self.glfw.poll_events();
            for (_, event) in glfw::flush_messages(&self.events) {
                Self::handle_window_event(&mut self.window, event);
            }
            if let Ok(fps) = recv.try_recv() {
                self.window.set_title(&format!("FPS: {:.2}", fps));
            }
        }

        // Wait for acknowledgement that the rendering was completed.
        let _ = render_task_done;
    }

    fn render_task(mut ctx: PRenderContext, game: Arc<Mutex<GameData>>, sender: Sender<f32>) {
        ctx.make_current();
        gl::load_with(|symbol| ctx.get_proc_address(symbol) as *const _);
        game.lock().expect("Could not lock game data in render thread").scene.as_mut().unwrap().init_gl();
        let mut fps_counter = FpsCounter::new();

        unsafe {
            gl::Enable(gl::MULTISAMPLE); // Enable multi-sampling
            gl::Enable(gl::BLEND); // Enable blending for better anti-aliasing
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA); // Set blending function
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Enable(gl::DEPTH_TEST);
        }
        let resolution = STATIC_DATA.read().expect("Failed to read config").config().get_resolution();
        let viewport = vec4(0.0, 0.0, resolution.0 as f32, resolution.1 as f32);

        loop {
            fps_counter.update();

            unsafe {
                gl::Viewport(viewport.x as i32, viewport.y as i32, viewport.z as i32, viewport.w as i32);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                let game = game.lock().expect("Could not lock game data in render thread");
                if let Some(scene) = &game.scene {
                    scene.render();
                }
            }

            ctx.swap_buffers();
            let _ = sender.send(fps_counter.fps());
        }
    }

    fn step_task(game: Arc<Mutex<GameData>>) {
        loop {
            {
                let mut game = game.lock().expect("Could not lock game data in step thread");
                if let Some(scene) = &mut game.scene {
                    scene.step();
                }
            }
            sleep(std::time::Duration::from_nanos(1));
        }
    }

    fn handle_window_event(window: &mut glfw::Window, event: WindowEvent) {
        match event {
            WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }
    }
}