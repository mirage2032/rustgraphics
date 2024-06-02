use std::sync::{Arc, Mutex};
use glam::Vec3;
use crate::engine::GameState;
use crate::engine::scene::gameobject::{GameObject, GameObjectData, GameObjectTrait};
use crate::engine::scene::gameobject::components::ComponentMap;
use crate::result::EngineStepResult;

pub struct RotatingGameObject {
    data: GameObjectData,
    rotation: Vec3,
    components: ComponentMap,
}

impl RotatingGameObject {
    pub fn new(parent: Option<GameObject>, rotation: Vec3) -> GameObject {
        let newgameobject = Arc::new(Mutex::new(Self {
            data: GameObjectData::new(parent.clone()),
            rotation,
            components: ComponentMap::new(),
        }));
        if let Some(parent) = parent {
            parent
                .lock()
                .expect("Could not lock parent gameobject for init")
                .data_mut()
                .children
                .push(newgameobject.clone());
        }
        newgameobject
    }
}

impl GameObjectTrait for RotatingGameObject {
    fn data(&self) -> &GameObjectData {
        &self.data
    }

    fn data_mut(&mut self) -> &mut GameObjectData {
        &mut self.data
    }
    
    fn components(&self) -> Option<&ComponentMap> {
        Some(&self.components)
    }
    
    fn components_mut(&mut self) -> Option<&mut ComponentMap> {
        Some(&mut self.components)
    }

    fn step(&mut self, game: &GameState) -> EngineStepResult<()> {
        let duration = game.delta.as_secs_f32();
        let rotation = self.rotation * duration;
        let data = self.data_mut();
        data.transform.rotation *= glam::Quat::from_rotation_x(rotation.x);
        data.transform.rotation *= glam::Quat::from_rotation_y(rotation.y);
        data.transform.rotation *= glam::Quat::from_rotation_z(rotation.z);
        Ok(())
    }
}
