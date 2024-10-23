use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use glam::vec3;
use crate::engine::scene::gameobject::GameObjectData;
use crate::engine::GameState;
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
}

impl ComponentMap {
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
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

    fn post_physics(&self, object: &mut GameObjectData) {
        if let Some(rigid_body) = self.get_component::<RigidBodyComponent>() {
            let rigid_body = rigid_body.borrow();
            object.transform.position = rigid_body.get_position();
            object.transform.rotation = rigid_body.get_rotation();
        }
    }
    fn pre_physics(&self, object: &mut GameObjectData) {
        if let Some(rigid_body) = self.get_component::<RigidBodyComponent>() {
            rigid_body.borrow_mut().set_transform(&object.transform);
        }
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
        self.pre_physics(object);
        Ok(())
    }
}
