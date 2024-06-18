use std::fs::File;
use std::io::{Read, Write};
use std::sync::RwLock;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    resolution: (u32, u32),
    fov: f32,
    near_clip: f32,
    far_clip: f32,
}

impl Config {
    pub fn new() -> Self {
        let config = Config::default();
        config
    }
    pub fn from_file(path: &str) -> Result<Self, &'static str> {
        let mut file = File::open(path).map_err(|_| "Failed to open file")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|_| "Failed to read file")?;
        let config: Config = serde_json::from_str(&contents).map_err(|_| "Failed to parse JSON")?;
        Ok(config)
    }

    pub fn to_file(&self, path: &str) -> Result<(), &'static str> {
        let json = serde_json::to_string(self).map_err(|_| "Failed to serialize JSON")?;
        let mut file = File::create(path).map_err(|_| "Failed to create file")?;
        file.write_all(json.as_bytes())
            .map_err(|_| "Failed to write file")?;
        Ok(())
    }
    pub fn with_resolution(mut self, resolution: (u32, u32)) -> Self {
        self.resolution = resolution;
        self
    }

    pub fn with_fov(mut self, fov: f32) -> Self {
        self.fov = fov;
        self
    }

    pub fn with_clip(mut self, near_clip: f32, far_clip: f32) -> Self {
        self.near_clip = near_clip;
        self.far_clip = far_clip;
        self
    }

    pub fn get_resolution(&self) -> (u32, u32) {
        self.resolution
    }

    pub fn get_resolution_x(&self) -> u32 {
        self.resolution.0
    }

    pub fn get_resolution_y(&self) -> u32 {
        self.resolution.1
    }

    pub fn get_fov(&self) -> f32 {
        self.fov
    }

    pub fn get_clip(&self) -> (f32, f32) {
        (self.near_clip, self.far_clip)
    }
}

impl Default for Config {
    fn default() -> Self {
        let default = Config {
            resolution: (1600, 900),
            fov: 70.0,
            near_clip: 0.1,
            far_clip: 300.0,
        };
        default
    }
}

pub struct StaticData {
    config: Config,
    projection: glam::Mat4,
}

impl StaticData {
    pub fn new() -> Self {
        let config = Config::new();
        StaticData::from_config(config)
    }

    pub fn from_config(config: Config) -> Self {
        let mut data = StaticData {
            config,
            projection: glam::Mat4::NAN,
        };
        data.calc_projection();
        data
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn set_config(&mut self, config: Config) {
        self.config = config;
        self.calc_projection();
    }

    fn calc_projection(&mut self) {
        let aspect_ratio = self.config.resolution.0 as f32 / self.config.resolution.1 as f32;
        self.projection = glam::Mat4::perspective_rh(
            self.config.fov.to_radians(),
            aspect_ratio,
            self.config.near_clip,
            self.config.far_clip,
        );
    }

    pub fn projection(&self) -> &glam::Mat4 {
        &self.projection
    }
}

impl Default for StaticData {
    fn default() -> Self {
        let config = Config::default();
        StaticData::from_config(config)
    }
}

lazy_static! {
    pub static ref CONFIG: RwLock<StaticData> = RwLock::new(StaticData::default());
}
