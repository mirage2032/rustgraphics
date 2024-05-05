use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::Builder;
use std::time::{Duration, Instant};
use glam::{Mat4, vec3, vec4};

use glfw::{Action, Context, Glfw, GlfwReceiver, Key, PWindow, WindowEvent, WindowHint};

use crate::{HEIGHT, WIDTH};
use crate::engine::camera::Camera;
use crate::engine::drawable::cube::DrawCube;
use crate::engine::drawable::Drawable;
use crate::engine::gameobject::{BaseGameObject, GameObject};
use crate::engine::scene::Scene;
use crate::engine::transform::Transform;

pub mod drawable;
pub mod shader;
pub mod gameobject;
pub mod transform;
pub mod scene;
mod camera;

pub struct Engine<'a> {
    glfw: Glfw,
    window: PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
    scene: Option<Scene<'a>>,
}

impl<'a> Engine<'a> {
    pub fn new() -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        glfw.window_hint(WindowHint::ContextVersion(3, 3));
        glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(WindowHint::Resizable(false));
        glfw.window_hint(WindowHint::Samples(Some(4))); // Set the number of samples for multi-sampling

        let (mut window, events) = glfw
            .create_window(*WIDTH, *HEIGHT, "Hello this is window", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_key_polling(true);


        Self { window, glfw, events,scene:None }
    }

    pub fn run(&mut self) {
        let render_context = self.window.render_context();
        let (send, recv) = channel();

        let render_task = Builder::new().name("render task".to_string());
        let render_task_done = render_task.spawn(move || {
            Self::render(render_context, send);
        });

        while !self.window.should_close() {
            self.glfw.poll_events();
            for (_, event) in glfw::flush_messages(&self.events) {
                Self::handle_window_event(&mut self.window, event);
            }
            if let Ok(val) =  recv.try_recv() {
                self.window.set_title(&format!("FPS: {:.2}", val));
            };
        }

        // Wait for acknowledgement that the rendering was completed.
        let _ = render_task_done;
    }

    fn render(mut ctx: glfw::PRenderContext, sender: Sender<f32>) {
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

        let viewport = vec4(0.0, 0.0, *WIDTH as f32, *HEIGHT as f32);

        loop {
            fps_counter.update();

            unsafe {
                gl::Viewport(viewport.x as i32, viewport.y as i32, *WIDTH as i32, *HEIGHT as i32);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                scene.render();
                scene.step();
            }

            ctx.swap_buffers();
            sender.send(fps_counter.fps()).expect("Failed to send FPS");
        }
    }

    fn handle_window_event(window: &mut glfw::Window, event: WindowEvent) {
        match event {
            WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }
    }
}

struct FpsCounter {
    frame_count: u32,
    start_time: Instant,
    fps: f32,
}

impl FpsCounter {
    fn new() -> Self {
        Self {
            frame_count: 0,
            start_time: Instant::now(),
            fps: 0.0,
        }
    }

    fn update(&mut self) {
        self.frame_count += 1;
        let elapsed = self.start_time.elapsed();
        if elapsed >= Duration::from_secs(1) {
            self.fps = self.frame_count as f32 / elapsed.as_secs_f32();
            self.frame_count = 0;
            self.start_time = Instant::now();
        }
    }

    fn fps(&self) -> f32 {
        self.fps
    }
}

