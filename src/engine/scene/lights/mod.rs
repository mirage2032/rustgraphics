use std::sync::{Mutex, Weak};

use gl::types::GLuint;

use directional::{DirectionalLight, DirectionalLightData};
use point::{PointLight, PointLightData};
use spot::{SpotLight, SpotLightData};

pub mod directional;
pub mod point;
pub mod spot;

const MAX_POINT_LIGHTS: usize = 4;
const MAX_SPOT_LIGHTS: usize = 4;
#[repr(C)]
#[derive(Debug, Clone)]
pub struct LightsData {
    pub is_directional: bool,
    pub directional: DirectionalLightData,
    pub point_count: u32,
    pub point: [PointLightData; MAX_POINT_LIGHTS],
    pub spot_count: u32,
    pub spot: [SpotLightData; MAX_SPOT_LIGHTS],
}

impl Default for LightsData {
    fn default() -> Self {
        Self {
            is_directional: false,
            directional: DirectionalLightData::empty(),
            point_count: 0,
            point: [PointLightData::empty(); MAX_POINT_LIGHTS],
            spot_count: 0,
            spot: [SpotLightData::empty(); MAX_SPOT_LIGHTS],
        }
    }
}
pub struct Lights {
    pub directional: Weak<Mutex<DirectionalLight>>,
    pub point: Vec<Weak<Mutex<PointLight>>>,
    pub spot: Vec<Weak<Mutex<SpotLight>>>,
    pub ssbo: GLuint,
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
            None => (DirectionalLightData::empty(), true),
        };

        // self.point.retain(|light| light.upgrade().is_some()); //CLEANUP
        let mut point = [PointLightData::empty(); MAX_POINT_LIGHTS];
        for (index, light) in self
            .point
            .iter()
            .filter_map(|light| light.upgrade())
            .enumerate()
        {
            point[index] = light
                .lock()
                .expect("Could not lock pointlight")
                .light_data();
        }

        // self.spot.retain(|light| light.upgrade().is_some()); //CLEANUP
        let mut spot = [SpotLightData::empty(); MAX_SPOT_LIGHTS];
        for (i, light) in self
            .spot
            .iter()
            .filter_map(|light| light.upgrade())
            .enumerate()
        {
            spot[i] = light.lock().expect("Could not lock spotlight").light_data();
        }
        LightsData {
            is_directional,
            directional,
            point_count: point.len() as u32,
            point,
            spot_count: spot.len() as u32,
            spot,
        }
    }

    pub fn init_ssbo(&mut self) {
        let size = std::mem::size_of::<LightsData>();
        let empty = LightsData::default();
        unsafe {
            gl::GenBuffers(1, &mut self.ssbo);
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, self.ssbo);
            gl::BufferData(
                gl::SHADER_STORAGE_BUFFER,
                size as isize,
                &empty as *const LightsData as *const std::ffi::c_void,
                gl::DYNAMIC_DRAW,
            );
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
        }
    }

    pub fn update_ssbo(&self) {
        let data = self.light_data();
        unsafe {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, self.ssbo);

            gl::BufferData(
                gl::SHADER_STORAGE_BUFFER,
                std::mem::size_of::<LightsData>() as isize,
                &data as *const LightsData as *const std::ffi::c_void,
                gl::DYNAMIC_DRAW,
            );

            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
        }
    }
    pub fn use_ssbo(&self, binding: u32) {
        unsafe {
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, binding, self.ssbo);
        }
    }
}

impl Drop for Lights {
    fn drop(&mut self) {
        if self.ssbo != 0 {
            unsafe {
                gl::DeleteBuffers(1, &self.ssbo);
            }
        }
    }
}

impl Default for Lights {
    fn default() -> Self {
        Self {
            directional: Weak::new(),
            point: Vec::new(),
            spot: Vec::new(),
            ssbo: 0,
        }
    }
}
