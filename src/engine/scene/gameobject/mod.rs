use std::cell::RefCell;
use std::rc::Rc;

use glam::Mat4;

use crate::engine::drawable::Drawable;
use crate::engine::scene::gameobject::components::drawable::DrawableComponent;
use crate::engine::scene::gameobject::components::ComponentMap;
use crate::engine::scene::lights::Lights;
use crate::engine::transform::Transform;
use crate::engine::GameState;
use crate::result::EngineStepResult;

pub mod base;
pub mod components;

pub trait GameObjectTrait: Drawable {
    fn data(&self) -> &GameObjectData;
    fn data_mut(&mut self) -> &mut GameObjectData;
    fn components(&self) -> Option<&ComponentMap>;
    fn components_mut(&mut self) -> Option<&mut ComponentMap>;
    fn step(&mut self, state: &GameState) -> EngineStepResult<()> { Ok(()) }
    fn step_recursive(&mut self, game: &GameState) -> EngineStepResult<()> {
        self.step(game)?;
        for child in &mut self.data_mut().children {
            child
                .borrow_mut()
                .step(game)?;
        }
        Ok(())
    }
    fn fixed_step(&mut self, state: &GameState) -> EngineStepResult<()> { Ok(()) }
    fn fixed_step_recursive(&mut self, game: &GameState) -> EngineStepResult<()> {
        self.fixed_step(game)?;
        for child in &mut self.data_mut().children {
            child
                .borrow_mut()
                .fixed_step(game)?;
        }
        Ok(())
    }
    fn global_mat(&self) -> Mat4 {
        let mut transform: Mat4 = self.data().transform.into();
        let mut parent = self.data().parent.clone();
        while let Some(parent_object) = parent {
            let parent_data = parent_object
                .borrow();
            transform = Mat4::from(parent_data.data().transform) * transform;
            parent = parent_data.data().parent.clone();
        }
        transform
    }
}

pub type GameObject = Rc<RefCell<dyn GameObjectTrait>>;

impl<T: GameObjectTrait> Drawable for T {
    fn draw(&mut self, modelmat: &Mat4, viewmat: &Mat4, lights: Option<&Lights>) {
        let data = self.data();
        let newmodelmat = *modelmat * Mat4::from(data.transform);
        if let Some(components) = self.components() {
            if let Some(drawable) = components.get_component::<DrawableComponent>() {
                drawable
                    .write()
                    .expect("Could not lock drawable component for draw")
                    .draw(&newmodelmat, viewmat, lights);
            }
        }

        for child in &data.children {
            child
                .borrow_mut()
                .draw(&newmodelmat, viewmat, lights);
        }
    }
}

pub struct GameObjectData {
    pub parent: Option<GameObject>,
    pub children: Vec<GameObject>,
    pub transform: Transform,
}

impl GameObjectData {
    pub fn new(parent: Option<GameObject>) -> Self {
        Self {
            parent,
            children: Vec::new(),
            transform: Transform::default(),
        }
    }
}
