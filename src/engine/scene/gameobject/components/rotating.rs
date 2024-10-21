use glam::Vec3;

use crate::engine::scene::gameobject::components::{Component, ComponentMap};
use crate::engine::scene::gameobject::GameObjectData;
use crate::engine::GameState;

pub struct RotatingComponent {
    pub direction: Vec3,
}

impl RotatingComponent {
    pub fn new(direction: Vec3) -> Self {
        Self { direction }
    }
}

impl Component for RotatingComponent {
    fn step(
        &mut self,
        data: &mut GameObjectData,
        _: &ComponentMap,
        game: &GameState,
    ) -> crate::result::EngineStepResult<()> {
        let duration = game.delta.as_secs_f32();
        let rotation = self.direction * duration;
        data.transform.rotation *= glam::Quat::from_rotation_x(rotation.x);
        data.transform.rotation *= glam::Quat::from_rotation_y(rotation.y);
        data.transform.rotation *= glam::Quat::from_rotation_z(rotation.z);
        Ok(())
    }
}
