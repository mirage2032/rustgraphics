use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, sync_channel, SyncSender};
use std::thread::Builder;
use std::time::Duration;
use std::ffi::CString;

use glfw::{
    Action, Context, Glfw, GlfwReceiver, Key, PRenderContext, PWindow, WindowEvent, WindowHint,
};

use crate::engine::config::STATIC_DATA;
use crate::engine::events::EngineInputsState;
use crate::engine::events::EngineWindowEvent;
use crate::engine::fpscounter::TimeDelta;
use crate::engine::scene::Scene;
use crate::result::{EngineRunError, EngineRunOut, EngineRunResult, EngineRenderResult, EngineRenderError,EngineStepResult};
use crate::result::EngineRunError::ThreadError;

pub mod camera;
pub mod components;
pub mod config;
pub mod drawable;
mod events;
pub mod fpscounter;
pub mod gameobject;
pub mod scene;
pub mod transform;

pub struct GameState {
    pub input_state: EngineInputsState,
    pub delta: Duration,
}

pub struct GameData {
    pub scene: Option<Box<dyn Scene>>,
    pub state: GameState,
}

impl GameData {
    pub fn new(scene: Option<Box<dyn Scene>>) -> Self {
        Self {
            scene,
            ..Default::default()
        }
    }

    fn step(&mut self, duration: Duration, input: EngineInputsState) -> EngineStepResult<()> {
        self.state.input_state.merge(input);
        self.state.delta = duration;
        if let Some(scene) = &mut self.scene {
            scene.step(&self.state)?;
        }
        Ok(())
    }
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            scene: None,
            state: GameState {
                input_state: EngineInputsState::new(),
                delta: Duration::new(0, 0),
            },
        }
    }
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
        glfw.window_hint(WindowHint::ContextVersion(4, 4));
        glfw.window_hint(WindowHint::CocoaGraphicsSwitching(false));
        glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
        glfw.window_hint(WindowHint::OpenGlDebugContext(true));
        glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(WindowHint::Resizable(false));
        glfw.window_hint(WindowHint::TransparentFramebuffer(true));
        glfw.window_hint(WindowHint::Samples(Some(4))); // Set the number of samples for multi-sampling

        let resolution = STATIC_DATA
            .read()
            .expect("Failed to read config")
            .config()
            .get_resolution();
        let (mut window, events) = glfw
            .create_window(
                resolution.0,
                resolution.1,
                "Hello this is window",
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window.");

        window.make_current(); // Print information about the GPU device
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        // Print information about the GPU device
        println!(
            "Renderer: {:?}",
            unsafe { std::ffi::CStr::from_ptr(gl::GetString(gl::RENDERER) as *const _) }
                .to_str()
                .unwrap()
        );
        println!(
            "Vendor: {:?}",
            unsafe { std::ffi::CStr::from_ptr(gl::GetString(gl::VENDOR) as *const _) }
                .to_str()
                .unwrap()
        );
        println!(
            "Version: {:?}",
            unsafe { std::ffi::CStr::from_ptr(gl::GetString(gl::VERSION) as *const _) }
                .to_str()
                .unwrap()
        );
        println!(
            "GLSL Version: {:?}",
            unsafe {
                std::ffi::CStr::from_ptr(gl::GetString(gl::SHADING_LANGUAGE_VERSION) as *const _)
            }
            .to_str()
            .unwrap()
        );

        window.set_key_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_mouse_button_polling(true);
        window.set_cursor_mode(glfw::CursorMode::Disabled);

        window.glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
        glfw.make_context_current(None);

        let game = Arc::new(Mutex::new(GameData {
            scene: None,
            ..Default::default()
        }));

        Self {
            window,
            game,
            events,
            glfw,
        }
    }

    pub fn from_game(game: GameData) -> Self {
        let mut engine = Self::new();
        engine.game = Arc::new(Mutex::new(game));
        engine
    }

    pub fn run(&mut self) -> EngineRunResult {
        let game = self.game.clone();
        let ctx = self.window.render_context();

        let render_task = Builder::new().name("render task".to_string());
        let (send_rend, recv_rend) = sync_channel(0);
        let render_task_done = render_task.spawn(move || Self::render_task(ctx, game, send_rend));

        let game = self.game.clone();
        let step_task = Builder::new().name("update task".to_string());
        let (send_step, recv_step) = sync_channel(0);
        let step_task_done = step_task.spawn(move || Self::step_task(game, recv_step));
        let exit_error: Option<EngineRunError> = {
            loop {
                if self.window.should_close() {
                    break None;
                }
                //check if render_rask still running and get result else
                match &render_task_done {
                    Ok(handle) => {
                        if handle.is_finished() {
                            break Some(ThreadError("Render task exited".to_string()))
                        }
                    }
                    Err(_) => break Some(ThreadError("No render task handle".to_string())),
                }

                match &step_task_done {
                    Ok(handle) => {
                        if handle.is_finished() {
                            break Some(ThreadError("Step task exited".to_string()))
                        }
                    }
                    Err(_) => break Some(ThreadError("No step task handle".to_string()))
                }

                if let Ok(delta) = recv_rend.try_recv() {
                    let seconds_per_frame = delta.as_secs_f64(); // Convert duration to seconds
                    let fps = {
                        if seconds_per_frame > 0.0 {
                            1.0 / seconds_per_frame // Calculate frames per second
                        } else {
                            0.0 // If duration is zero or negative, return 0 FPS to avoid division by zero
                        }
                    };
                    self.window.set_title(&format!("FPS: {:.2}", fps));
                    self.glfw.poll_events();
                    let (engine_events, mut input_changes) =
                        self.gather_window_events();
                    input_changes.mouse_pos = self.window.get_cursor_pos();
                    let mut exit_event = None;
                    for event in engine_events {
                        match event {
                            EngineWindowEvent::Close => {
                                exit_event = Some(None);
                                break;
                            }
                            _ => {}
                        }
                    }
                    if let Some(reason) = exit_event {
                        break reason
                    }
                    if let Err(_) = send_step.send((delta, input_changes)) {
                        break  Some(ThreadError("Could not send step".to_string()));
                    }
                }
            }
        };
        self.window.set_should_close(true);
        drop(recv_rend);
        drop(send_step);

        let mut exit_status = EngineRunOut::new();
        exit_status.render_result = match render_task_done {
            Ok(result) => {
                match result.join() {
                    Ok(result) => result,
                    Err(_) => Err(EngineRenderError::JoinThreadError),
                }
            },
            Err(_) => Ok(()),
        };
        exit_status.step_result = match step_task_done {
            Ok(result) => {
                match result.join() {
                    Ok(result) => result,
                    Err(_) => Err("Could not join step thread".to_string()),
                }
            },
            Err(_) => Ok(()),
        };
        if let Some(e) = exit_error {
            exit_status.main_result = Err(e);
            return Err(exit_status);
        }
        Ok(())
    }

    fn render_task(
        mut ctx: PRenderContext,
        game: Arc<Mutex<GameData>>,
        sender: SyncSender<Duration>,
    ) -> EngineRenderResult<()> {
        ctx.make_current();
        game.lock()
            .expect("Could not lock game data in render thread")
            .scene
            .as_mut()
            .unwrap()
            .init_gl()?;
        let mut fps_counter = TimeDelta::new();

        unsafe {
            gl::Enable(gl::MULTISAMPLE); // Enable multi-sampling
            gl::Enable(gl::BLEND); // Enable blending for better anti-aliasing
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA); // Set blending function
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::BACK);
            gl::Enable(gl::DEBUG_OUTPUT);
            gl::DebugMessageCallback(Some(debug_callback), std::ptr::null());
        }
        

        loop {
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                let game = game
                    .lock()
                    .expect("Could not lock game data in render thread");
                if let Some(scene) = &game.scene {
                    scene.render();
                }
            }
            ctx.swap_buffers();
            if let Err(_) = sender.send(fps_counter.delta()) {
                break;
            }
        }
        Ok(())
    }

    fn step_task(
        game: Arc<Mutex<GameData>>,
        sender: Receiver<(Duration, EngineInputsState)>,
    ) -> EngineStepResult<()> {
        while let Ok((delta, changes)) = sender.recv() {
            let game_clone = game.clone();
            let mut game_locked = game_clone
                .lock()
                .map_err(|_|"Could not lock game data in step thread")?;
            game_locked.step(delta, changes)?;
        }
        Ok(())
    }

    fn gather_window_events(&self) -> (Vec<EngineWindowEvent>, EngineInputsState) {
        let mut engine_events = vec![];
        let mut input_changes = EngineInputsState::new();
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    engine_events.push(EngineWindowEvent::Close)
                }
                WindowEvent::Key(key, _, action, _) => {
                    input_changes.keyboard.add_key(key, action);
                }
                WindowEvent::MouseButton(button, action, _) => {
                    input_changes.mouse.add_key(button, action);
                }
                _ => {}
            }
        }
        (engine_events, input_changes)
    }
}


extern "system" fn debug_callback(
    _source: gl::types::GLenum,
    _type: gl::types::GLenum,
    _id: gl::types::GLuint,
    severity: gl::types::GLenum,
    _length: gl::types::GLsizei,
    message: *const gl::types::GLchar,
    _user_param: *mut std::ffi::c_void,
) {
    if severity == gl::DEBUG_SEVERITY_HIGH || severity == gl::DEBUG_SEVERITY_MEDIUM {
        unsafe {
            let error_message = CString::from_raw(message as *mut i8);
            println!("OpenGL Error: {:?}", error_message);
        }
    }
}