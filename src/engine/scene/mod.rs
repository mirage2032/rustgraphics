use glam::{Mat4, vec3};

use crate::engine::camera::Camera;
use crate::engine::gameobject::GameObject;

pub struct SceneData {
    pub objects: Vec<Box<dyn GameObject<'static>>>,
    pub main_camera: Option<Camera>,
}

pub trait Scene: Send {
    fn data(&self) -> &SceneData;
    fn data_mut(&mut self) -> &mut SceneData;
    fn init_gl(&mut self);
    fn render(&self){
        if let Some(camera) = &self.data().main_camera {
            let viewmat = camera.transform.to_mat4();
            for object in &self.data().objects {
                object.draw(&Mat4::from_translation(vec3(0.0,0.0,0.0)), &viewmat);
            }
        }
    }
    fn step(&mut self) {
        for object in &mut self.data_mut().objects {
            object.step();
        }
    }
}