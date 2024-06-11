use std::mem;
use std::sync::{RwLock, Weak};

use gl::types::{GLsizeiptr, GLuint};
use glsl_layout::{boolean, int, Std140, Uniform};

use directional::{DirectionalLight, DirectionalLightData};
use point::{PointLight, PointLightData};
use spot::{SpotLight, SpotLightData};

pub mod directional;
pub mod point;
pub mod spot;

const MAX_POINT_LIGHTS: usize = 5;
const MAX_SPOT_LIGHTS: usize = 5;
#[derive(Debug, Copy,Default, Clone, Uniform)]
pub struct LightsData {
    pub is_directional: boolean,
    pub directional: DirectionalLightData,
    pub point_count: int,
    pub point: [PointLightData; MAX_POINT_LIGHTS],
    pub spot_count: int,
    pub spot: [SpotLightData; MAX_SPOT_LIGHTS],
}

pub struct Lights {
    pub directional: Weak<RwLock<DirectionalLight>>,
    pub point: Vec<Weak<RwLock<PointLight>>>,
    pub spot: Vec<Weak<RwLock<SpotLight>>>,
    pub ssbo: GLuint,
}

impl Lights {
    pub fn light_data(&mut self) -> LightsData {
        let (directional, is_directional) = match &self.directional.upgrade() {
            Some(light) => (
                light
                    .read()
                    .expect("Could not lock directional light")
                    .light_data(),
                true,
            ),
            None => (DirectionalLightData::empty(), false),
        };

        let mut point = [PointLightData::empty(); MAX_POINT_LIGHTS];
        let mut point_count = 0;
        self.point.retain(|light|
            {
                if let Some(light) = light.upgrade() {
                    point[point_count] = light.read().expect("Could not lock pointlight").light_data();
                    point_count += 1;
                    true
                } else {
                    false
                }
            }
        );

        let mut spot = [SpotLightData::empty(); MAX_SPOT_LIGHTS];
        let mut spot_count = 0;
        self.spot.retain(|light|
            {
                if let Some(light) = light.upgrade() {
                    spot[spot_count] = light.read().expect("Could not lock spotlight").light_data();
                    spot_count += 1;
                    true
                } else {
                    false
                }
            }
        );
        LightsData {
            is_directional: is_directional.into(),
            directional,
            point_count: point_count as i32,
            point ,
            spot_count: spot_count as i32,
            spot,
        }
    }

    pub fn init_ssbo(&mut self) {
        let empty = LightsData::default();
        unsafe {
            gl::GenBuffers(1, &mut self.ssbo);
            gl::BindBuffer(gl::UNIFORM_BUFFER, self.ssbo);
            gl::BufferData(
                gl::UNIFORM_BUFFER,
                mem::size_of::<LightsData>() as GLsizeiptr,
                empty.std140().as_raw().as_ptr() as *const _,
                gl::DYNAMIC_DRAW,
            );
            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
        }
    }

    pub fn update_ssbo(&mut self) {
        let data = self.light_data();
        unsafe {
            gl::BindBuffer(gl::UNIFORM_BUFFER, self.ssbo);

            gl::BufferSubData(
                gl::UNIFORM_BUFFER,
                0,
                mem::size_of::<LightsData>() as GLsizeiptr,
                data.std140().as_raw().as_ptr() as *const _,
            );

            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
        }
    }
    pub fn bind(&self, binding: u32) {
        unsafe {
            gl::BindBufferBase(gl::UNIFORM_BUFFER, binding, self.ssbo);
        }
    }

    pub fn unbind(binding: u32) {
        unsafe {
            gl::BindBufferBase(gl::UNIFORM_BUFFER, binding, 0);
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
