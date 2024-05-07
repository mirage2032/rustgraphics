use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender};
use std::thread::Builder;

use glam::{Mat4, vec3, vec4};
use glfw::{Action, Context, Glfw, GlfwReceiver, Key, PRenderContext, PWindow, WindowEvent, WindowHint};

use crate::engine::camera::Camera;
use crate::engine::config::{STATIC_DATA};
use crate::engine::drawable::cube::DrawCube;
use crate::engine::drawable::Drawable;
use crate::engine::fpscounter::FpsCounter;
use crate::engine::gameobject::{BaseGameObject, GameObject};
use crate::engine::scene::Scene;
use crate::engine::transform::Transform;

pub mod drawable;
pub mod shader;
pub mod gameobject;
pub mod transform;
pub mod scene;
pub mod camera;
pub mod fpscounter;
pub mod config;

pub struct GameData {
    pub scene: Option<Scene>,
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

        Self { window, game, events, glfw}
    }

    pub fn run(&mut self) {
        let (send, recv) = channel();

        let render_task = Builder::new().name("render task".to_string());
        let game = self.game.clone();
        let ctx = self.window.render_context();
        let render_task_done = render_task.spawn(move || {
            Self::render_task(ctx, game, send);
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

        let mut fps_counter = FpsCounter::new();

        let mut scene = {
            let mut scene = Scene::new();
            let mut cubeobj: Box<dyn GameObject> = Box::new(BaseGameObject::new(None));
            cubeobj.data_mut().drawable = Some(Box::new(DrawCube::default()));
            scene.objects.push(cubeobj);

            let rotated_view = {
                let eye = vec3(0.0, 0.0, 3.0); // Camera position
                let center = vec3(0.0, 0.0, 0.0); // Camera target
                let up = vec3(0.0, 1.0, 0.0); // Up vector

                // Create a view matrix
                let view = Mat4::look_at_rh(eye, center, up);

                // Define rotation parameters
                let angle = 15.0_f32.to_radians(); // Rotation angle in degrees
                let axis = vec3(0.0, 1.0, 0.0); // Rotation axis

                // Create a rotation matrix
                let rotation = Mat4::from_axis_angle(axis, angle);
                view * rotation
            };
            let camera = Camera::new(Transform::from(rotated_view));
            scene.main_camera = Some(camera);
            scene
        };


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
                scene.render();
                scene.step();
            }

            ctx.swap_buffers();
            let _ = sender.send(fps_counter.fps());
        }
    }

    fn handle_window_event(window: &mut glfw::Window, event: WindowEvent) {
        match event {
            WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }
    }
}