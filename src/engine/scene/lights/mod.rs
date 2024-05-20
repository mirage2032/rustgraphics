use std::sync::{Arc, Mutex};

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
    pub directional: Option<Arc<Mutex<DirectionalLight>>>,
    pub point: Vec<Arc<Mutex<PointLight>>>,
    pub spot: Vec<Arc<Mutex<SpotLight>>>,
}

impl Lights {
    pub fn light_data(&self) -> LightsData {
        let (directional,is_directional) = match &self.directional {
            Some(light) => (light
                .lock()
                .expect("Could not lock directional light")
                .light_data(),true),
            None => (DirectionalLightData::empty(),false),
        };
        let point = self
            .point
            .iter()
            .map(|light| {
                light
                    .lock()
                    .expect("Could not lock point light")
                    .light_data()
            })
            .collect();
        let spot = self
            .spot
            .iter()
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
            directional: None,
            point: Vec::new(),
            spot: Vec::new(),
        }
    }
}
