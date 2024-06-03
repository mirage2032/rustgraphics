use std::sync::{Mutex, RwLock, Weak};

use glam::{Mat4, vec3};

use crate::engine::GameState;
use crate::engine::scene::camera::CameraControlled;
use crate::engine::scene::gameobject::{GameObject, GameObjectTrait};
use crate::result::{EngineRenderResult, EngineStepResult};

pub mod camera;
pub mod gameobject;
pub mod lights;

pub struct SceneData {
    pub objects: Vec<GameObject>,
    pub main_camera: Weak<RwLock<CameraControlled>>,
    pub lights: lights::Lights,
}

pub trait Scene: Send {
    fn data(&self) -> &SceneData;
    fn data_mut(&mut self) -> &mut SceneData;
    fn init_gl(&mut self) -> EngineRenderResult<()>;
    fn render(&mut self) {
        if let Some(camera) = &self.data().main_camera.upgrade() {
            let camera_mat = camera
                .read()
                .expect("Could not lock camera for render")
                .global_mat();
            
            let viewmat: Mat4 = camera_mat.inverse();
            self.data_mut().lights.update_ssbo();
            for object in &self.data().objects {
                object
                    .read()
                    .expect("Could not lock gameobject for draw")
                    .draw(
                        &Mat4::from_translation(vec3(0.0, 0.0, 0.0)),
                        &viewmat,
                        Some(&self.data().lights),
                    );
            }
        }
    }
    fn step(&mut self, state: &GameState) -> EngineStepResult<()> {
        for object in &self.data_mut().objects {
            object
                .write()
                .expect("Could not lock gameobject for step")
                .step_recursive(state)?;
        }
        Ok(())
    }
}
