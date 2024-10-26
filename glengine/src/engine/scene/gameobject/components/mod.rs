use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::rc::Rc;
use crate::engine::GameState;
use crate::engine::scene::gameobject::base::GameObjectData;
use crate::engine::scene::gameobject::components::rigidbody::RigidBodyComponent;
use crate::result::EngineStepResult;

pub mod freecam;
pub mod drawable;
pub mod rotating;
pub mod rigidbody;
pub mod collider;

pub trait Component{
    fn setup(&mut self, _object: &mut GameObjectData, _components: &ComponentMap){}
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
    elements: HashMap<TypeId, Rc<RefCell<Box<dyn Component>>>>,
    transform_hash: u64,
}

impl ComponentMap {
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
            transform_hash: 0,
        }
    }

    pub fn add_component<T: Component+'static>(&mut self, mut component: T, object: &mut GameObjectData) {
        let type_id = TypeId::of::<T>();
        component.setup(object, self);
        self.elements
            .insert(type_id, Rc::new(RefCell::new(Box::new(component))));
    }

    pub fn get_component<T: Component+'static>(&self) -> Option<Rc<RefCell<Box<T>>>> {
        let type_id = TypeId::of::<T>();
        self.elements.get(&type_id).and_then(|rc| {
            Some(unsafe { (*(rc as *const Rc<RefCell<Box<dyn Component>>> as *const Rc<RefCell<Box<T>>>)).clone() })
        })
    }
    fn apply_transform_to_physics(&mut self, object: &mut GameObjectData) {
        if let Some(rigid_body) = self.get_component::<RigidBodyComponent>() {
            if self.transform_hash != Self::calculate_transform_hash(object) {
                // println!("Applying transform to physics");
                rigid_body.borrow_mut().set_transform(&object.transform);
            }
        }
    }

    fn calculate_transform_hash(object:&mut GameObjectData) -> u64 {
        let mut hasher = DefaultHasher::new();
        object.transform.hash(&mut hasher);
        hasher.finish()
    }
    pub fn step(&mut self, object: &mut GameObjectData, state: &GameState) -> EngineStepResult<()> {
        self.transform_hash = Self::calculate_transform_hash(object);
        for (_, component) in self.elements.iter() {
            component
                .borrow_mut()
                .step(object, self, state)?
        }
        Ok(())
    }
    pub fn fixed_step(&mut self, object: &mut GameObjectData, state: &GameState) -> EngineStepResult<()> {
        for (_, component) in self.elements.iter() {
            component
                .borrow_mut()
                .fixed_step(object, self, state)?
        }
        self.apply_transform_to_physics(object);
        Ok(())
    }
}
