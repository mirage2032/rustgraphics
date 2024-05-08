use std::time::Duration;
use glam::{Vec3, Mat4};
use obj::Position;
use crate::engine::gameobject::{GameObject,GameObjectData,GameObjectRaw};
use crate::engine::transform::Transform;

pub struct PointCamera {
    pub data: GameObjectData,
}

impl PointCamera {
    pub fn new(parent:Option<GameObject>,position: Vec3,target: Vec3,up:Vec3) -> Self {
        let mut data = GameObjectData::new(parent);
        data.transform.position = position;
        data.transform = Mat4::look_at_rh(position, target, up).into();
        Self {
            data: data,
        }
    }
}

impl Default for PointCamera {
    fn default() -> Self {
        Self {
            data: GameObjectData::new(None),
        }
    }
}

impl GameObjectRaw for PointCamera {
    fn data(&self) -> &GameObjectData {
        &self.data
    }

    fn data_mut(&mut self) -> &mut GameObjectData {
        &mut self.data
    }

    fn step(&mut self, duration: &Duration) {
    }
}