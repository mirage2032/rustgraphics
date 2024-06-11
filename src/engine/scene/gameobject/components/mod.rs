use std::any::{TypeId};
use std::sync::RwLock;

use indexmap::IndexMap;

use crate::engine::GameState;
use crate::engine::scene::gameobject::GameObjectData;
use crate::result::EngineStepResult;

pub mod freecam;
pub mod drawable;
pub mod rotating;


pub trait Component: Send + Sync + 'static{
    fn step(
        &mut self,
        _object: &mut GameObjectData,
        _components: &ComponentMap,
        _state: &GameState,
    ) -> EngineStepResult<()> {
        Ok(())}
}

pub struct ComponentMap {
    elements: IndexMap<TypeId, RwLock<Box<dyn Component>>>,
}

impl ComponentMap {
    pub fn new() -> Self {
        Self {
            elements: IndexMap::new(),
        }
    }

    pub fn add_component<T: Component>(&mut self, component: T) {
        let type_id = TypeId::of::<T>();
        self.elements
            .insert(type_id, RwLock::new(Box::new(component)));
    }

    pub fn get_component<T: Component>(&self) -> Option<&RwLock<Box<T>>> {
        let type_id = TypeId::of::<T>();
        self.elements.get(&type_id).and_then(|mutex| {
            Some(unsafe { &*(mutex as *const RwLock<Box<dyn Component>> as *const RwLock<Box<T>>) })
        })
    }
    pub fn step(&self, object: &mut GameObjectData, state: &GameState) -> EngineStepResult<()> {
        for (_, component) in self.elements.iter() {
            component
                .write()
                .expect("Could not lock component for step")
                .step(object, self, state)?
        }
        Ok(())
    }
}
