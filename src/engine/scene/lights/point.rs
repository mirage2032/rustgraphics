use std::sync::{Arc, Mutex};

use glam::Vec3;
use glsl_layout::{float, Uniform, vec3};

use crate::engine::scene::gameobject::{GameObject, GameObjectData, GameObjectTrait};
use crate::engine::scene::gameobject::components::ComponentMap;

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
    data: GameObjectData,
    components: ComponentMap,
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
    ) -> Arc<Mutex<Self>> {
        let light = Arc::new(Mutex::new(Self {
            data: GameObjectData::new(parent),
            components: ComponentMap::new(),
            intensity,
            color,
            constant,
            linear,
            quadratic,
        }));
        if let Some(parent) = &light.lock().unwrap().data.parent {
            parent
                .lock()
                .unwrap()
                .data_mut()
                .children
                .push(light.clone());
        }
        light
    }

    pub fn light_data(&self) -> PointLightData {
        let position = self.glob_pos();
        PointLightData {
            intensity: self.intensity,
            color: vec3::from([self.color.x, self.color.y, self.color.z]),
            position: vec3::from([position.x, position.y, position.z]),
            constant: self.constant,
            linear: self.linear,
            quadratic: self.quadratic,
        }
    }
}

impl GameObjectTrait for PointLight {
    fn step(&mut self, state: &crate::engine::GameState) -> crate::result::EngineStepResult<()> {
        self.components.step(&mut self.data, state)?;
        Ok(())
    }
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
}
