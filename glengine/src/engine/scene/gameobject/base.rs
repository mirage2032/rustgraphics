use std::cell::RefCell;
use std::rc::Rc;
use glam::Mat4;
use crate::engine::drawable::Drawable;
use crate::engine::scene::gameobject::components::{Component, ComponentMap};
use crate::engine::scene::gameobject::{GameObject, GameObjectData};
use crate::engine::GameState;
use crate::engine::scene::gameobject::components::collider::ColliderComponent;
use crate::engine::scene::gameobject::components::drawable::DrawableComponent;
use crate::engine::scene::gameobject::components::rigidbody::RigidBodyComponent;
use crate::engine::scene::lights::Lights;
use crate::engine::transform::Transform;
use crate::result::EngineStepResult;

pub struct BaseGameObject {
    pub data: GameObjectData,
    pub components: ComponentMap,
}

impl BaseGameObject {
    pub fn new(parent: Option<GameObject>) -> GameObject {
        let newgameobject = Rc::new(RefCell::new(Self {
            data: GameObjectData::new(parent.clone()),
            components: ComponentMap::new(),
        }));
        if let Some(parent) = parent {
            parent
                .borrow_mut()
                .data
                .children
                .push(newgameobject.clone());
        }
        newgameobject
    }

    pub fn new_w_transform(parent: Option<GameObject>,trasform:Transform) -> GameObject {
        let newgameobject = Rc::new(RefCell::new(Self {
            data: GameObjectData::new_w_transform(parent.clone(),trasform),
            components: ComponentMap::new(),
        }));
        if let Some(parent) = parent {
            parent
                .borrow_mut()
                .data
                .children
                .push(newgameobject.clone());
        }
        newgameobject
    }

    pub fn step(&mut self, state: &GameState) -> EngineStepResult<()> {
        self.components.step(&mut self.data, state)?;
        for child in &mut self.data.children {
            child
                .borrow_mut()
                .step(state)?;
        }
        Ok(())
    }
    pub fn fixed_step(&mut self, state: &GameState, physics_components: &mut Vec<(Rc<RefCell<Box<RigidBodyComponent>>>,Rc<RefCell<Box<ColliderComponent>>>)>) -> EngineStepResult<()> {
        self.components.fixed_step(&mut self.data, state)?;
        for child in &mut self.data.children {
            child
                .borrow_mut()
                .fixed_step(state,physics_components)?;
        }
        if let (Some(rigid_body),Some(collider)) = (self.components.get_component::<RigidBodyComponent>(),self.components.get_component::<ColliderComponent>()) {
            physics_components.push((rigid_body, collider));
        }
        Ok(())
    }
    
    pub fn add_component<T: Component+'static >(&mut self, component: T) {
        self.components.add_component(component, &mut self.data);
    }
    pub fn global_mat(&self) -> Mat4 {
        let mut transform: Mat4 = self.data.transform.into();
        let mut parent = self.data.parent.clone();
        while let Some(parent_object) = parent {
            let parent_data = parent_object
                .borrow();
            transform = Mat4::from(parent_data.data.transform) * transform;
            parent = parent_data.data.parent.clone();
        }
        transform
    }
}

impl Drawable for BaseGameObject {
    fn draw(&mut self, modelmat: &Mat4, viewmat: &Mat4, lights: Option<&Lights>) {
        let newmodelmat = *modelmat * Mat4::from(self.data.transform);
        if let Some(drawable) = self.components.get_component::<DrawableComponent>() {
            drawable
                .borrow_mut()
                .draw(&newmodelmat, viewmat, lights);
        }

        for child in &self.data.children {
            child
                .borrow_mut()
                .draw(&newmodelmat, viewmat, lights);
        }
    }
}
