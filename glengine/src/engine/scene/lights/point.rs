use std::cell::RefCell;
use std::rc::{Rc, Weak};

use glam::Vec3;
use glsl_layout::{float, vec3, Uniform};

use crate::engine::scene::gameobject::components::ComponentMap;
use crate::engine::scene::gameobject::{GameObject};
use crate::engine::scene::gameobject::base::{BaseGameObject, GameObjectData};
use crate::engine::transform::Transform;

#[derive(Debug, Copy, Default, Clone, Uniform)]
pub struct PointLightData {
    pub intensity: float,
    pub color: vec3,
    pub position: vec3,
    pub constant: float,
    pub linear: float,
    pub quadratic: float,
}

impl PointLightData {
    pub fn empty() -> Self {
        Self {
            intensity: 0.0,
            color: vec3::from([0.0, 0.0, 0.0]),
            position: vec3::from([0.0, 0.0, 0.0]),
            constant: 0.0,
            linear: 0.0,
            quadratic: 0.0,
        }
    }
}
pub struct PointLight {
    pub game_object: Weak<RefCell<BaseGameObject>>,
    pub intensity: f32,
    pub color: Vec3,
    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
}

impl PointLight {
    pub fn new(
        parent: Option<GameObject>,
        intensity: f32,
        color: Vec3,
        constant: f32,
        linear: f32,
        quadratic: f32,
    ) -> Self {
        let game_object = GameObject{base:Rc::new(RefCell::new(BaseGameObject {
            data: GameObjectData::new(parent.clone()),
            components: ComponentMap::new(),
        }))};
        let light = Self {
            game_object: Rc::downgrade(&game_object.base),
            intensity,
            color,
            constant,
            linear,
            quadratic,
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
    
    pub fn new_w_gameobject(game_object: GameObject, intensity: f32, color: Vec3, constant: f32, linear: f32, quadratic: f32) -> Self {
        let light = Self {
            game_object: Rc::downgrade(&game_object.base),
            intensity,
            color,
            constant,
            linear,
            quadratic,
        };
        light
    }

    pub fn light_data(&self) -> Option<PointLightData> {
        let transform:Transform = self.game_object.upgrade()?.borrow().global_mat().into();
        let position = transform.position;
        Some(PointLightData {
            intensity: self.intensity,
            color: vec3::from([self.color.x, self.color.y, self.color.z]),
            position: vec3::from([position.x, position.y, position.z]),
            constant: self.constant,
            linear: self.linear,
            quadratic: self.quadratic,
        })
    }
}