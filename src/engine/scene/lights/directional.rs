use glam::Vec3;
use glsl_layout::{float, Uniform, vec3};
use crate::engine::scene::gameobject::{GameObject, GameObjectData, GameObjectTrait};
use crate::engine::scene::gameobject::components::ComponentMap;

#[derive(Debug, Copy,Default, Clone,Uniform)]
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
            direction: vec3::from([0.0, 0.0, 0.0])
        }
    }
}

pub struct DirectionalLight {
    data: GameObjectData,
    components: ComponentMap,
    pub intensity: f32,
    pub color: Vec3,
}
impl DirectionalLight {
    pub fn new(parent: Option<GameObject>, intensity: f32, color: Vec3) -> Self {
        let data = GameObjectData::new(parent);
        Self {
            data,
            components: ComponentMap::new(),
            intensity,
            color,
        }
    }

    pub fn light_data(&self) -> DirectionalLightData {
        let direction = self.data.transform.forward();
        DirectionalLightData {
            intensity: self.intensity,
            color: vec3::from([self.color.x, self.color.y, self.color.z]),
            direction: vec3::from([direction.x, direction.y, direction.z])
        }
    }
}

impl GameObjectTrait for DirectionalLight {
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
    
    fn step(&mut self, state: &crate::engine::GameState) -> crate::result::EngineStepResult<()> {
        self.components.step(&mut self.data, state)?;
        Ok(())
    }
    
}