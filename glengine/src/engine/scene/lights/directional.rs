use std::cell::RefCell;
use std::rc::{Rc, Weak};

use glam::Vec3;
use glsl_layout::{float, vec3, Uniform};

use crate::engine::scene::gameobject::components::ComponentMap;
use crate::engine::scene::gameobject::{GameObject};
use crate::engine::scene::gameobject::base::{BaseGameObject, GameObjectData};

#[derive(Debug, Copy, Default, Clone, Uniform)]
pub struct DirectionalLightData {
    pub intensity: float,
    pub color: vec3,
    pub direction: vec3,
}

impl DirectionalLightData {
    pub fn empty() -> Self {
        Self {
            intensity: 0.0,
            color: vec3::from([0.0, 0.0, 0.0]),
            direction: vec3::from([0.0, 0.0, 0.0]),
        }
    }
}

pub struct DirectionalLight {
    pub game_object: Weak<RefCell<BaseGameObject>>,
    pub intensity: f32,
    pub color: Vec3,
}
impl DirectionalLight {
    pub fn new(parent: Option<GameObject>, intensity: f32, color: Vec3) -> Self {
        let game_object = GameObject{base:Rc::new(RefCell::new(BaseGameObject{
            data: GameObjectData::new(parent.clone()),
            components: ComponentMap::new(),
        }))};
        let light = Self {
            game_object: Rc::downgrade(&game_object.base),
            intensity,
            color,
        };
        if let Some(parent) = parent {
            parent.base
                .borrow_mut()
                .data
                .children
                .push(game_object.clone());
        }
        light
    }

    pub fn new_w_gameobject(game_object: GameObject, intensity: f32, color: Vec3) -> Self {
        let light = Self {
            game_object: Rc::downgrade(&game_object.base),
            intensity,
            color,
        };
        light
    }

    pub fn light_data(&self) -> Option<DirectionalLightData> {
        let direction = self.game_object.upgrade()?.borrow().data.transform.forward();
        Some(DirectionalLightData {
            intensity: self.intensity,
            color: vec3::from([self.color.x, self.color.y, self.color.z]),
            direction: vec3::from([direction.x, direction.y, direction.z]),
        })
    }
}
