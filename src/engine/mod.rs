use std::ffi::CString;
use std::time::Duration;
use glam::Mat4;
use gl;

use glfw::{
    Action, Context, Glfw, GlfwReceiver, Key, PRenderContext, PWindow, WindowEvent, WindowHint,
};


use crate::engine::config::CONFIG;
use crate::engine::drawable::{Drawable};
use crate::engine::events::EngineInputsState;
use crate::engine::events::EngineWindowEvent;
use crate::engine::fbo::{ScreenFbo};
use crate::engine::scene::Scene;
use crate::result::{EngineRenderResult, EngineRunError, EngineRunResult, EngineStepResult};

pub mod config;
pub mod drawable;
pub mod events;

pub mod fbo;
pub mod fpscounter;
pub mod scene;
pub mod transform;
// lazy_static! {
//     pub static ref vr_context: Mutex<openvr::Context> =
//         unsafe { Mutex::new(openvr::init(openvr::ApplicationType::Scene).unwrap()) };
//     pub static ref vr_system: openvr::System = vr_context.lock().unwrap().system().unwrap();
//     pub static ref vr_compositor: openvr::Compositor =
//         vr_context.lock().unwrap().compositor().unwrap();
// }

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

    fn step(&mut self, duration: Duration) -> EngineStepResult<()> {
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
    game: GameData,
    events: GlfwReceiver<(f64, WindowEvent)>,
    glfw: Glfw,
}

impl Engine {
    pub fn new() -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
        glfw.window_hint(WindowHint::ContextVersion(3, 0));
        // glfw.window_hint(WindowHint::RefreshRate(Some(0)));
        glfw.window_hint(WindowHint::CocoaGraphicsSwitching(false));
        glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
        glfw.window_hint(WindowHint::OpenGlDebugContext(true));
        glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(WindowHint::ClientApi(glfw::ClientApiHint::OpenGlEs));
        glfw.window_hint(WindowHint::Resizable(false));
        glfw.window_hint(WindowHint::TransparentFramebuffer(true));
        // glfw.window_hint(WindowHint::Samples(Some(16))); // Set the number of samples for multi-sampling

        let resolution = CONFIG
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
        let mut maxsamples = 0;
        unsafe {
            gl::GetIntegerv(gl::MAX_SAMPLES, &mut maxsamples);
        }
        println!(
            "Max samples: {:?}",
            maxsamples
        );

        window.set_key_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_mouse_button_polling(true);
        window.set_cursor_mode(glfw::CursorMode::Disabled);

        window.glfw.set_swap_interval(glfw::SwapInterval::Adaptive);
        // unsafe {
        //     glfwSwapInterval(0);
        // }
        glfw.make_context_current(None);

        let game = GameData {
            scene: None,
            ..Default::default()
        };
        Self {
            window,
            game,
            events,
            glfw,
        }
    }

    pub fn from_game(game: GameData) -> Self {
        let mut engine = Self::new();
        engine.game = game;
        engine
    }

    pub fn run(&mut self) ->EngineRunResult{
        self.window.make_current();
        let mut render_ctx=self.window.render_context();
        self.gl_init().expect("Could not init gl");
        let resolution = CONFIG.read()
            .expect("Failed to read config")
            .config()
            .get_resolution();
        let mut mainfbo = ScreenFbo::new(resolution.0, resolution.1,8);
        loop {
            self.render(&mut mainfbo, &mut render_ctx);
            self.handle_events();
            self.step(Duration::from_secs_f64(1.0/60.0))
                .map_err(|err|EngineRunError::StepError(err))?;
        }
        Ok(())
    }
    fn gl_init(
        &mut self,
    ) -> EngineRenderResult<()> {
        self.window.make_current();
        if let Some(scene) = &mut self.game.scene{
            scene.init_gl()?;
        }
        unsafe {
            // gl::Enable(gl::MULTISAMPLE); // Enable multi-sampling
            gl::Enable(gl::BLEND); // Enable blending for better anti-aliasing
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA); // Set blending function
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::BACK);
            gl::Enable(gl::DEBUG_OUTPUT);
            gl::DebugMessageCallback(Some(debug_callback), std::ptr::null());
        }
        Ok(())
    }

    fn render(
        &mut self,
        screen_fbo: &mut ScreenFbo,
        ctx: &mut PRenderContext,
    ){
        if let Some(ref mut scene) = &mut self.game.scene {
            screen_fbo.render(scene);
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            }
            screen_fbo.draw_data.draw(&Mat4::ZERO, &Mat4::ZERO, None);
        }
        ctx.swap_buffers();
    }
    fn handle_events(&mut self){
        self.glfw.poll_events();
        let (engine_events, input_changes) = self.gather_window_events();
        for event in engine_events {
            match event {
                EngineWindowEvent::Close => {
                    self.window.set_should_close(true);
                }
                _ => {}
            }
        }
        self.game.state.input_state.merge(input_changes);
    }
    fn step(
        &mut self,
        delta: Duration,
    ) -> EngineStepResult<()> {
        self.game.step(delta)?;
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
        input_changes.mouse_pos = self.window.get_cursor_pos();
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
