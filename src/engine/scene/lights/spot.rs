use glam::Vec3;
use crate::engine::scene::gameobject::{GameObject, GameObjectData, GameObjectRaw};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SpotLightData {
    pub intensity: f32,
    pub color: Vec3,
    pub position: Vec3,
    pub direction: Vec3,
    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
    pub cut_off: f32,
    pub outer_cut_off: f32,
}

impl SpotLightData {
    pub fn empty() -> Self {
        Self {
            intensity: 0.0,
            color: Vec3::ZERO,
            position: Vec3::ZERO,
            direction: Vec3::ZERO,
            constant: 0.0,
            linear: 0.0,
            quadratic: 0.0,
            cut_off: 0.0,
            outer_cut_off: 0.0,
        }
    }
}

pub struct SpotLight {
    data: GameObjectData,
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
        parent: GameObject,
        intensity: f32,
        color: Vec3,
        constant: f32,
        linear: f32,
        quadratic: f32,
        cut_off: f32,
        outer_cut_off: f32,
    ) -> Self {
        let data = GameObjectData::new(Some(parent));
        Self {
            data,
            intensity,
            color,
            constant,
            linear,
            quadratic,
            cut_off,
            outer_cut_off,
        }
    }

    pub fn light_data(&self) -> SpotLightData {
        let position = self.data.transform.position;
        let direction = self.data.transform.forward();
        SpotLightData {
            intensity: self.intensity,
            color: self.color,
            position,
            direction,
            constant: self.constant,
            linear: self.linear,
            quadratic: self.quadratic,
            cut_off: self.cut_off,
            outer_cut_off: self.outer_cut_off,
        }
    }
}

impl GameObjectRaw for SpotLight {
    fn data(&self) -> &GameObjectData {
        &self.data
    }

    fn data_mut(&mut self) -> &mut GameObjectData {
        &mut self.data
    }
}
