use std::cell::RefCell;
use std::rc::Weak;

use glam::{vec3, Mat4};

use crate::engine::scene::camera::CameraControlled;
use crate::engine::scene::gameobject::{GameObject, GameObjectTrait};
use crate::engine::GameState;
use crate::result::{EngineRenderResult, EngineStepResult};

pub mod camera;
pub mod gameobject;
pub mod lights;

pub struct SceneData {
    pub objects: Vec<GameObject>,
    pub main_camera: Weak<RefCell<CameraControlled>>,
    pub lights: lights::Lights,
}

pub trait Scene {
    fn data(&self) -> &SceneData;
    fn data_mut(&mut self) -> &mut SceneData;
    fn init_gl(&mut self) -> EngineRenderResult<()>;
    fn render(&mut self) {
        if let Some(camera) = &self.data().main_camera.upgrade() {
            let camera_mat = camera
                .borrow()
                .global_mat();
            
            let viewmat: Mat4 = camera_mat.inverse();
            self.data_mut().lights.update_ssbo();
            for object in &self.data().objects {
                object
                    .borrow_mut()
                    .draw(
                        &Mat4::from_translation(vec3(0.0, 0.0, 0.0)),
                        &viewmat,
                        Some(&self.data().lights),
                    );
            }
        }
    }
    
    fn step(&mut self,state: &GameState) -> EngineStepResult<()> {Ok(())}
    fn step_recursive(&mut self, state: &GameState) -> EngineStepResult<()> {
        self.step(state)?;
        for object in &self.data_mut().objects {
            object
                .borrow_mut()
                .step_recursive(state)?;
        }
        Ok(())
    }
    
    fn fixed_step(&mut self,state: &GameState) -> EngineStepResult<()> {Ok(())}
    fn fixed_step_recursive(&mut self, state: &GameState) -> EngineStepResult<()> {
        self.fixed_step(state)?;
        for object in &self.data_mut().objects {
            object
                .borrow_mut()
                .fixed_step_recursive(state)?;
        }
        Ok(())
    }
}
