use std::any::{Any, TypeId};
use std::sync::Mutex;

use indexmap::IndexMap;

use crate::engine::GameState;
use crate::engine::scene::gameobject::GameObjectData;
use crate::result::EngineStepResult;

pub mod freecam;

pub trait Component: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn step(
        &mut self,
        object: &mut GameObjectData,
        components: &ComponentMap,
        state: &GameState,
    ) -> EngineStepResult<()>;
}

pub struct ComponentMap {
    elements: IndexMap<TypeId, Mutex<Box<dyn Component>>>,
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
            .insert(type_id, Mutex::new(Box::new(component)));
    }

    pub fn get_component<T: Component>(&self) -> Option<&Mutex<Box<T>>> {
        let type_id = TypeId::of::<T>();
        self.elements.get(&type_id).and_then(|mutex| {
            Some(unsafe { &*(mutex as *const Mutex<Box<dyn Component>> as *const Mutex<Box<T>>) })
        })
    }
    pub fn step(&self, object: &mut GameObjectData, state: &GameState) -> EngineStepResult<()> {
        for (_, component) in self.elements.iter() {
            component
                .lock()
                .expect("Could not lock component")
                .step(object, self, state)?
        }
        Ok(())
    }
}
