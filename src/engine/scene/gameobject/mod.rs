use std::sync::{Arc, Mutex};

use glam::{Mat4, Vec3};

use crate::engine::drawable::Draw;
use crate::engine::GameState;
use crate::engine::scene::gameobject::components::ComponentMap;
use crate::engine::scene::lights::Lights;
use crate::engine::transform::Transform;
use crate::result::EngineStepResult;

pub mod base;
pub mod components;
pub mod rotating;

pub trait GameObjectTrait: Draw + Send {
    fn data(&self) -> &GameObjectData;
    fn data_mut(&mut self) -> &mut GameObjectData;
    fn components(&self) -> Option<&ComponentMap>;
    fn components_mut(&mut self) -> Option<&mut ComponentMap>;
    fn step(&mut self, state: &GameState) -> EngineStepResult<()>;
    fn step_recursive(&mut self, game: &GameState) -> EngineStepResult<()> {
        self.step(game)?;
        for child in &mut self.data_mut().children {
            child
                .lock()
                .expect("Could not lock child gameobject for step")
                .step(game)?;
        }
        Ok(())
    }
    fn global_mat(&self) -> Mat4 {
        let mut transform: Mat4 = self.data().transform.into();
        let mut parent = self.data().parent.clone();
        while let Some(parent_object) = parent {
            let parent_data = parent_object
                .lock()
                .expect("Could not lock parent gameobject for global transform");
            transform = Mat4::from(parent_data.data().transform) * transform;
            parent = parent_data.data().parent.clone();
        }
        transform
    }

    fn glob_pos(&self) -> Vec3 {
        let mut position = self.data().transform.position;
        let mut parent = self.data().parent.clone();
        while let Some(parent_object) = parent {
            let parent_data = parent_object
                .lock()
                .expect("Could not lock parent gameobject for global transform");
            position = parent_data.data().transform.position + position;
            parent = parent_data.data().parent.clone();
        }
        position
    }
}

pub type GameObject = Arc<Mutex<dyn GameObjectTrait>>;

impl<T: GameObjectTrait> Draw for T {
    fn draw(&self, modelmat: &Mat4, viewmat: &Mat4, lights: Option<&Lights>) {
        let data = self.data();
        let newmodelmat = *modelmat * Mat4::from(data.transform);
        if let Some(drawable) = &data.drawable {
            drawable.draw(&newmodelmat, viewmat, lights);
        }
        for child in &data.children {
            child
                .lock()
                .expect("Could not lock child gameobject for draw")
                .draw(&newmodelmat, viewmat, lights);
        }
    }
}

pub struct GameObjectData {
    pub parent: Option<GameObject>,
    pub children: Vec<GameObject>,
    pub transform: Transform,
    pub drawable: Option<Box<dyn Draw>>,
}

impl GameObjectData {
    pub fn new(parent: Option<GameObject>) -> Self {
        Self {
            parent,
            children: Vec::new(),
            transform: Transform::default(),
            drawable: None,
        }
    }
}
