use std::any::{Any, TypeId};
use std::collections::HashMap;

use crate::engine::GameState;
use crate::engine::scene::gameobject::GameObjectData;
use crate::result::EngineStepResult;

pub mod freecam;

pub trait Component: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn step(&mut self, object: &mut GameObjectData, state: &GameState) -> EngineStepResult<()>;
}

pub struct ComponentMap {
    elements: HashMap<TypeId, Box<dyn Component>>,
}

impl ComponentMap {
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
        }
    }

    pub fn add_component<T: Component>(&mut self, component: T) {
        let type_id = TypeId::of::<T>();
        self.elements.insert(type_id, Box::new(component));
    }

    pub fn get_component<T: Component>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.elements
            .get(&type_id)
            .map(|component| component.as_any().downcast_ref::<T>().unwrap())
    }

    pub fn get_component_mut<T: Component>(&mut self) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        self.elements
            .get_mut(&type_id)
            .map(|component| component.as_any_mut().downcast_mut::<T>().unwrap())
    }

    pub fn step(&mut self, object: &mut GameObjectData, state: &GameState) -> EngineStepResult<()> {
        for (_, component) in self.elements.iter_mut() {
            component.step(object, state)?
        }
        Ok(())
    }
}
