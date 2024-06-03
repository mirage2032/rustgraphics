use glam::Vec3;
use glsl_layout::{float, Uniform, vec3};

use crate::engine::scene::gameobject::{GameObject, GameObjectData, GameObjectTrait};
use crate::engine::scene::gameobject::components::ComponentMap;

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
    data: GameObjectData,
    components: ComponentMap,
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
            components: ComponentMap::new(),
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
            color: vec3::from([self.color.x, self.color.y, self.color.z]),
            position: vec3::from([position.x, position.y, position.z]),
            direction: vec3::from([direction.x, direction.y, direction.z]),
            constant: self.constant,
            linear: self.linear,
            quadratic: self.quadratic,
            cut_off: self.cut_off,
            outer_cut_off: self.outer_cut_off,
        }
    }
}

impl GameObjectTrait for SpotLight {
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
