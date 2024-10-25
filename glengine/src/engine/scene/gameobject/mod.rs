use std::cell::RefCell;
use std::rc::Rc;
use glam::Mat4;
use crate::engine::drawable::Drawable;
use crate::engine::GameState;
use crate::engine::transform::Transform;
use crate::engine::scene::gameobject::base::{BaseGameObject, GameObjectData};
use crate::engine::scene::gameobject::components::collider::ColliderComponent;
use crate::engine::scene::gameobject::components::Component;
use crate::engine::scene::gameobject::components::rigidbody::RigidBodyComponent;
use crate::result::EngineStepResult;

pub mod base;
pub mod components;

#[derive(Clone)]
pub struct GameObject{
    pub base:Rc<RefCell<BaseGameObject>>
}

impl GameObject{
    pub fn new(parent:Option<GameObject>)->Self{
        let newgameobject = GameObject{base:Rc::new(RefCell::new(BaseGameObject {
            data: GameObjectData::new(parent.clone()),
            components: components::ComponentMap::new(),
        }))};
        if let Some(parent) = parent {
            parent
                .base
                .borrow_mut()
                .data
                .children
                .push(newgameobject.clone());
        }
        newgameobject
    }

    pub fn new_w_transform(parent:Option<GameObject>,transform:Transform)->Self{
        let newgameobject = GameObject{base:Rc::new(RefCell::new(BaseGameObject {
            data: GameObjectData::new_w_transform(parent.clone(),transform),
            components: components::ComponentMap::new(),
        }))};
        if let Some(parent) = parent {
            parent
                .base
                .borrow_mut()
                .data
                .children
                .push(newgameobject.clone());
        }
        newgameobject
    }

    pub fn step(&mut self, state: &GameState) -> EngineStepResult<()> {
        self.base.borrow_mut().step(state)
    }

    pub fn fixed_step(&mut self, state: &GameState, physics_components: &mut Vec<GameObject>) -> EngineStepResult<()> {
        let mut base = self.base.borrow_mut();
        if base.components.get_component::<RigidBodyComponent>().is_some() && base.components.get_component::<ColliderComponent>().is_some(){
            physics_components.push(self.clone());
        }
        base.fixed_step(state,physics_components)
    }
    pub fn add_component<T: Component+'static >(&mut self, component: T) {
        self.base.borrow_mut().add_component(component);
    }

    pub fn global_mat(&self) -> Mat4 {
        self.base.borrow().global_mat()
    }
}
