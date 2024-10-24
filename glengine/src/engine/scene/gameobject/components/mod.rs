use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::engine::scene::gameobject::GameObjectData;
use crate::engine::GameState;
use crate::result::EngineStepResult;

pub mod freecam;
pub mod drawable;
pub mod rotating;


pub trait Component{
    fn step(
        &mut self,
        _object: &mut GameObjectData,
        _components: &ComponentMap,
        _state: &GameState,
    ) -> EngineStepResult<()> {
        Ok(())}
    fn fixed_step(
        &mut self,
        _object: &mut GameObjectData,
        _components: &ComponentMap,
        _state: &GameState,
    ) -> EngineStepResult<()> {
        Ok(())}
}

pub struct ComponentMap {
    elements: HashMap<TypeId, RefCell<Box<dyn Component>>>,
}

impl ComponentMap {
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
        }
    }

    pub fn add_component<T: Component+'static>(&mut self, component: T) {
        let type_id = TypeId::of::<T>();
        self.elements
            .insert(type_id, RefCell::new(Box::new(component)));
    }

    pub fn get_component<T: Component+'static>(&self) -> Option<&RefCell<Box<T>>> {
        let type_id = TypeId::of::<T>();
        self.elements.get(&type_id).and_then(|mutex| {
            Some(unsafe { &*(mutex as *const RefCell<Box<dyn Component>> as *const RefCell<Box<T>>) })
        })
    }
    pub fn step(&self, object: &mut GameObjectData, state: &GameState) -> EngineStepResult<()> {
        for (_, component) in self.elements.iter() {
            component
                .borrow_mut()
                .step(object, self, state)?
        }
        Ok(())
    }
    pub fn fixed_step(&self, object: &mut GameObjectData, state: &GameState) -> EngineStepResult<()> {
        for (_, component) in self.elements.iter() {
            component
                .borrow_mut()
                .fixed_step(object, self, state)?
        }
        Ok(())
    }
}
