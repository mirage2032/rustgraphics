use crate::engine::camera::Camera;
use crate::engine::gameobject::GameObject;

pub struct Scene<'a> {
    objects: Vec<GameObject<'a>>,
    main_camera: Option<Camera>,
}

impl<'a> Scene<'a> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            main_camera: None,
        }
    }

}