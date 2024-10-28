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

pub struct Scene {
    pub objects: Vec<GameObject>,
    pub main_camera: Option<Camera>,
    pub lights: lights::Lights,
    init_gl_cb: fn(&mut Scene) -> EngineRenderResult<()>
}

impl Scene {
    pub fn new(init_gl_cb:fn(&mut Scene) -> EngineRenderResult<()>)-> Scene{
        Scene{
            objects:vec![],
            main_camera:None,
            lights:lights::Lights::default(),
            init_gl_cb
        }
    }
    pub fn init_gl(&mut self) -> EngineRenderResult<()>{
        (self.init_gl_cb)(self)
    }
    pub fn render(&mut self) {
        if let Some(camera) = &self.main_camera {
            let camera_mat = camera.game_object.global_mat();
            
            let viewmat: Mat4 = camera_mat.inverse();
            self.lights.update_ssbo();
            for object in &self.objects {
                object.base.borrow_mut().draw(
                        &Mat4::from_translation(vec3(0.0, 0.0, 0.0)),
                        &viewmat,
                        Some(&self.lights),
                    );
            }
        }
    }
    pub fn step_recursive(&mut self, state: &GameState) -> EngineStepResult<()> {
        for object in &mut self.objects {
            object
                .step(state)?;
        }
        Ok(())
    }
    
    pub fn fixed_step(&mut self, state: &GameState, physics_components: &mut Vec<GameObject>) -> EngineStepResult<()> {
        for object in &mut self.objects {
            object
                .fixed_step(state,physics_components)?;
        }
        Ok(())
    }
}
