use std::sync::{Mutex, Weak};

use directional::{DirectionalLight, DirectionalLightData};
use point::{PointLight, PointLightData};
use spot::{SpotLight, SpotLightData};

pub mod directional;
pub mod point;
pub mod spot;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct LightsData {
    pub is_directional: bool,
    pub directional: DirectionalLightData,
    pub point: Vec<PointLightData>,
    pub spot: Vec<SpotLightData>,
}
pub struct Lights {
    pub directional: Weak<Mutex<DirectionalLight>>,
    pub point: Vec<Weak<Mutex<PointLight>>>,
    pub spot: Vec<Weak<Mutex<SpotLight>>>,
}

impl Lights {
    pub fn light_data(&self) -> LightsData {
        let (directional, is_directional) = match &self.directional.upgrade() {
            Some(light) => (
                light
                    .lock()
                    .expect("Could not lock directional light")
                    .light_data(),
                true,
            ),
            None => (DirectionalLightData::empty(), false),
        };
        let point = self
            .point
            .iter()
            .filter_map(|light| light.upgrade())
            .map(|light| {
                light
                    .lock()
                    .expect("Could not lock pointlight")
                    .light_data()
            })
            .collect();
        let spot = self
            .spot
            .iter()
            .filter_map(|light| light.upgrade())
            .map(|light| light.lock().expect("Could not lock spotlight").light_data())
            .collect();
        LightsData {
            is_directional,
            directional,
            point,
            spot,
        }
    }
}

impl Default for Lights {
    fn default() -> Self {
        Self {
            directional: Weak::new(),
            point: Vec::new(),
            spot: Vec::new(),
        }
    }
}
