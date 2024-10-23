use std::cell::RefCell;
use std::rc::{Rc, Weak};

use glam::{vec3, Mat4};
use crate::engine::drawable::Drawable;
use crate::engine::scene::camera::Camera;
use crate::engine::scene::gameobject::{GameObject};
use crate::engine::GameState;
use crate::engine::scene::gameobject::components::collider::ColliderComponent;
use crate::engine::scene::gameobject::components::rigidbody::RigidBodyComponent;
use crate::result::{EngineRenderResult, EngineStepResult};

pub mod camera;
pub mod gameobject;
pub mod lights;

pub struct SceneData {
    pub objects: Vec<GameObject>,
    pub main_camera: Option<Camera>,
    pub lights: lights::Lights,
}

pub trait Scene {
    fn data(&self) -> &SceneData;
    fn data_mut(&mut self) -> &mut SceneData;
    fn init_gl(&mut self) -> EngineRenderResult<()>;
    fn render(&mut self) {
        if let Some(camera) = &self.data().main_camera {
            let camera_mat = camera.game_object
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
    fn step_recursive(&mut self, state: &GameState) -> EngineStepResult<()> {
        for object in &self.data_mut().objects {
            object
                .borrow_mut()
                .step(state)?;
        }
        Ok(())
    }
    
    fn fixed_step(&mut self, state: &GameState, physics_components: &mut Vec<(Rc<RefCell<Box<RigidBodyComponent>>>,Rc<RefCell<Box<ColliderComponent>>>)>) -> EngineStepResult<()> {
        for object in &self.data_mut().objects {
            object
                .borrow_mut()
                .fixed_step(state,physics_components)?;
        }
        Ok(())
    }
}
