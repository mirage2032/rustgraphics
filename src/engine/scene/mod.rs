use glam::Mat4;

use crate::engine::camera::Camera;
use crate::engine::gameobject::GameObject;

pub struct Scene<'a> {
    objects: Vec<Box<dyn GameObject<'a>>>,
    main_camera: Option<Camera>,
}

impl<'a> Scene<'a> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            main_camera: None,
        }
    }

    pub fn render(&self) {
        if let Some(camera) = &self.main_camera {
            let viewmat = camera.transform.to_mat4().inverse();
            for object in &self.objects {
                object.draw(&Mat4::IDENTITY, &viewmat);
            }
        }
    }
}