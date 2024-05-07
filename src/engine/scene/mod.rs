use glam::{Mat4, vec3};

use crate::engine::camera::Camera;
use crate::engine::gameobject::GameObject;

pub struct Scene {
    pub objects: Vec<Box<dyn GameObject<'static>>>,
    pub main_camera: Option<Camera>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            main_camera: None,
        }
    }

    pub fn render(&self) {
        if let Some(camera) = &self.main_camera {
            let viewmat = camera.transform.to_mat4();
            for object in &self.objects {
                object.draw(&Mat4::from_translation(vec3(0.0,0.0,0.0)), &viewmat);
            }
        }
    }

    pub fn step(&mut self) {
        for object in &mut self.objects {
            object.step();
        }
    }

}