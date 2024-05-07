use glam::{Mat4,Vec3};
use std::time::Duration;

use crate::engine::drawable::Drawable;
use crate::engine::transform::Transform;

pub trait GameObject<'a>: Drawable + 'a {
    fn data(&self) -> &GameObjectData<'a>;
    fn data_mut(&mut self) -> &mut GameObjectData<'a>;
    fn step(&mut self,duration: &Duration) {
        for child in &mut self.data_mut().children {
            child.step(duration);
        }
    }
}

impl<'a, T: GameObject<'a>> Drawable for T {
    fn draw(&self, modelmat: &Mat4, viewmat: &Mat4) {
        let data = self.data();
        let newmodelmat = *modelmat * data.transform.to_mat4();
        if let Some(drawable) = &data.drawable {
            drawable.draw(&newmodelmat, viewmat);
        }
        for child in &data.children {
            child.draw(&newmodelmat, viewmat);
        }
    }
}


pub struct GameObjectData<'a> {
    pub parent: Option<&'a dyn GameObject<'a>>,
    pub children: Vec<Box<dyn GameObject<'a>>>,
    pub transform: Transform,
    pub drawable: Option<Box<dyn Drawable>>,
}

impl<'a> GameObjectData<'a> {
    pub fn new(parent: Option<&'a dyn GameObject<'a>>) -> Self {
        Self {
            parent,
            children: Vec::new(),
            transform: Transform::default(),
            drawable: None,
        }
    }
}

pub struct BaseGameObject<'a> {
    data: GameObjectData<'a>,
    rotation: Vec3,
}

impl<'a> BaseGameObject<'a> {
    pub fn new(parent: Option<&'a dyn GameObject<'a>>,rotation: Vec3) -> Self {
        Self {
            data: GameObjectData::new(parent),
            rotation
        }
    }
}

impl<'a> GameObject<'a> for BaseGameObject<'a> {
    fn data(&self) -> &GameObjectData<'a> {
        &self.data
    }

    fn data_mut(&mut self) -> &mut GameObjectData<'a> {
        &mut self.data
    }

    fn step(&mut self,duration: &Duration) {
        let rotation = self.rotation * duration.as_secs_f32();
        let data = self.data_mut();
        data.transform.rotation *= glam::Quat::from_rotation_x(rotation.x);
        data.transform.rotation *= glam::Quat::from_rotation_y(rotation.y);
        data.transform.rotation *= glam::Quat::from_rotation_z(rotation.z);
        for child in &mut self.data_mut().children {
            child.step(duration);
        }
    }
}