use glam::Vec3;
use crate::engine::scene::gameobject::{GameObject, GameObjectData, GameObjectRaw};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PointLightData {
    pub intensity: f32,
    pub color: Vec3,
    pub position: Vec3,
    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
}

impl PointLightData {
    pub fn empty() -> Self {
        Self {
            intensity: 0.0,
            color: Vec3::ZERO,
            position: Vec3::ZERO,
            constant: 0.0,
            linear: 0.0,
            quadratic: 0.0,
        }
    }
}
pub struct PointLight {
    data: GameObjectData,
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
        let data = GameObjectData::new(parent);
        Self {
            data,
            intensity,
            color,
            constant,
            linear,
            quadratic,
        }
    }

    pub fn light_data(&self) -> PointLightData {
        let position = self.data.transform.position;
        PointLightData {
            intensity: self.intensity,
            color: self.color,
            position,
            constant: self.constant,
            linear: self.linear,
            quadratic: self.quadratic,
        }
    }
    
}

impl GameObjectRaw for PointLight {
    fn data(&self) -> &GameObjectData {
        &self.data
    }

    fn data_mut(&mut self) -> &mut GameObjectData {
        &mut self.data
    }
}