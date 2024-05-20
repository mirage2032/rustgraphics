use glam::Vec3;
use crate::engine::scene::gameobject::{GameObject, GameObjectData, GameObjectRaw};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DirectionalLightData {
    pub intensity: f32,
    pub color: Vec3,
    pub direction: Vec3,
}

impl DirectionalLightData {
    pub fn empty() -> Self {
        Self {
            intensity: 0.0,
            color: Vec3::ZERO,
            direction: Vec3::ZERO,
        }
    }
}

pub struct DirectionalLight {
    data: GameObjectData,
    pub intensity: f32,
    pub color: Vec3,
}
impl DirectionalLight {
    pub fn new(parent: Option<GameObject>, intensity: f32, color: Vec3) -> Self {
        let data = GameObjectData::new(parent);
        Self {
            data,
            intensity,
            color,
        }
    }

    pub fn light_data(&self) -> DirectionalLightData {
        let direction = self.data.transform.forward();
        DirectionalLightData {
            intensity: self.intensity,
            color: self.color,
            direction,
        }
    }
}

impl GameObjectRaw for DirectionalLight {
    fn data(&self) -> &GameObjectData {
        &self.data
    }

    fn data_mut(&mut self) -> &mut GameObjectData {
        &mut self.data
    }
}