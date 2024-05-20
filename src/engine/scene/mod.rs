use std::sync::{Arc, Mutex};

use glam::{Mat4, vec3};

use crate::engine::GameState;
use crate::engine::scene::camera::CameraControlled;
use crate::engine::scene::gameobject::{GameObject, GameObjectRaw};
use crate::result::{EngineRenderResult, EngineStepResult};

pub mod camera;
pub mod gameobject;
pub mod lights;

pub struct SceneData {
    pub objects: Vec<GameObject>,
    pub main_camera: Option<Arc<Mutex<CameraControlled>>>,
    pub lights: lights::Lights,
}

pub trait Scene: Send {
    fn data(&self) -> &SceneData;
    fn data_mut(&mut self) -> &mut SceneData;
    fn init_gl(&mut self) -> EngineRenderResult<()>;
    fn render(&self) {
        if let Some(camera) = &self.data().main_camera {
            let camera_mat = camera
                .lock()
                .expect("Could not lock camera for render")
                .global_mat();
            let viewmat: Mat4 = Mat4::from(camera_mat).inverse();
            for object in &self.data().objects {
                object
                    .lock()
                    .expect("Could not lock gameobject for draw")
                    .draw(&Mat4::from_translation(vec3(0.0, 0.0, 0.0)), &viewmat);
            }
        }
    }
    fn step(&mut self, state: &GameState) -> EngineStepResult<()> {
        for object in &mut self.data_mut().objects {
            object
                .lock()
                .expect("Could not lock gameobject for step")
                .step_recursive(state)?;
        }
        Ok(())
    }
}
