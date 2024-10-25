use glam::Vec3;
use glsl_layout::{float, vec3, Uniform};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::engine::scene::gameobject::components::ComponentMap;
use crate::engine::scene::gameobject::{GameObject};
use crate::engine::scene::gameobject::base::{BaseGameObject, GameObjectData};
use crate::engine::transform::Transform;

#[derive(Debug, Copy, Default, Clone, Uniform)]
pub struct SpotLightData {
    pub intensity: float,
    pub color: vec3,
    pub position: vec3,
    pub direction: vec3,
    pub constant: float,
    pub linear: float,
    pub quadratic: float,
    pub cut_off: float,
    pub outer_cut_off: float,
}

impl SpotLightData {
    pub fn empty() -> Self {
        Self {
            intensity: 0.0,
            color: vec3::from([0.0, 0.0, 0.0]),
            position: vec3::from([0.0, 0.0, 0.0]),
            direction: vec3::from([0.0, 0.0, 0.0]),
            constant: 0.0,
            linear: 0.0,
            quadratic: 0.0,
            cut_off: 0.0,
            outer_cut_off: 0.0,
        }
    }
}

pub struct SpotLight {
    pub game_object: Weak<RefCell<BaseGameObject>>,
    pub intensity: f32,
    pub color: Vec3,
    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
    pub cut_off: f32,
    pub outer_cut_off: f32,
}

impl SpotLight {
    pub fn new(
        parent: Option<GameObject>,
        intensity: f32,
        color: Vec3,
        constant: f32,
        linear: f32,
        quadratic: f32,
        cut_off: f32,
        outer_cut_off: f32,
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
            cut_off,
            outer_cut_off,
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
    
    pub fn new_w_gameobject(game_object: GameObject, intensity: f32, color: Vec3, constant: f32, linear: f32, quadratic: f32, cut_off: f32, outer_cut_off: f32) -> Self {
        let light = Self {
            game_object: Rc::downgrade(&game_object.base),
            intensity,
            color,
            constant,
            linear,
            quadratic,
            cut_off,
            outer_cut_off,
        };
        light
    }

    pub fn light_data(&self) -> Option<SpotLightData> {
        let mat:Transform = self.game_object.upgrade()?.borrow().global_mat().into();
        let position = mat.position;
        let direction = mat.forward();
        
        Some(SpotLightData {
            intensity: self.intensity,
            color: vec3::from([self.color.x, self.color.y, self.color.z]),
            position: vec3::from([position.x, position.y, position.z]),
            direction: vec3::from([direction.x, direction.y, direction.z]),
            constant: self.constant,
            linear: self.linear,
            quadratic: self.quadratic,
            cut_off: self.cut_off,
            outer_cut_off: self.outer_cut_off,
        })
    }
}