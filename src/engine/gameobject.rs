use glam::{Mat4,Vec3};
use std::time::Duration;
use std::sync::{Arc, RwLock};

use crate::engine::drawable::Drawable;
use crate::engine::transform::Transform;

pub trait GameObjectRaw: Drawable {
    fn data(&self) -> &GameObjectData;
    fn data_mut(&mut self) -> &mut GameObjectData;
    fn step(&mut self,duration: &Duration) {
        for child in &mut self.data_mut().children {
            child.write().expect("Could not lock child gameobject for step").step(duration);
        }
    }
}

pub type GameObject = Arc<RwLock<dyn GameObjectRaw>>;

impl<T: GameObjectRaw> Drawable for T {
    fn draw(&self, modelmat: &Mat4, viewmat: &Mat4) {
        let data = self.data();
        let newmodelmat = *modelmat * data.transform.to_mat4();
        if let Some(drawable) = &data.drawable {
            drawable.draw(&newmodelmat, viewmat);
        }
        for child in &data.children {
            child.read().expect("Could not lock child gameobject for draw").draw(&newmodelmat, viewmat);
        }
    }
}


pub struct GameObjectData {
    pub parent: Option<GameObject>,
    pub children: Vec<GameObject>,
    pub transform: Transform,
    pub drawable: Option<Box<dyn Drawable>>,
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

pub struct BaseGameObject {
    data: GameObjectData,
    rotation: Vec3,
}

impl BaseGameObject {
    pub fn new(parent: Option<GameObject>, rotation: Vec3) -> Self {
        Self {
            data: GameObjectData::new(parent),
            rotation
        }
    }
}

impl GameObjectRaw for BaseGameObject {
    fn data(&self) -> &GameObjectData {
        &self.data
    }

    fn data_mut(&mut self) -> &mut GameObjectData {
        &mut self.data
    }

    fn step(&mut self,duration: &Duration) {
        let rotation = self.rotation * duration.as_secs_f32();
        let data = self.data_mut();
        data.transform.rotation *= glam::Quat::from_rotation_x(rotation.x);
        data.transform.rotation *= glam::Quat::from_rotation_y(rotation.y);
        data.transform.rotation *= glam::Quat::from_rotation_z(rotation.z);
        for child in &mut self.data_mut().children {
            child.write().expect("Could not lock child gameobject for step").step(duration);
        }
    }
}