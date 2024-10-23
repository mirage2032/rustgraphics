use std::cell::RefCell;
use std::rc::Rc;
use glam::{Mat4, Vec3, Vec4};

use crate::engine::config::CONFIG;
use crate::engine::scene::gameobject::components::{freecam, ComponentMap};
use crate::engine::scene::gameobject::{GameObject, GameObjectData};
use crate::engine::scene::gameobject::base::BaseGameObject;

pub struct Camera {
    pub game_object: GameObject,
}

impl Camera {
    pub fn new(parent: Option<GameObject>, position: Vec3, target: Vec3, up: Vec3) -> Self {
        let mut data = GameObjectData::new(parent);
        data.transform = Mat4::look_at_rh(position, target, up).inverse().into();
        let mut components = ComponentMap::new();
        components.add_component(freecam::FreeCamComponent::new(),&mut data);
        Self {
            game_object: Rc::new(RefCell::new(BaseGameObject {
                data,
                components,
            }))
        }
    }

    pub fn frustum(&self) -> Mat4 {
        let perspective = *CONFIG.projection();
        perspective * self.game_object.borrow().global_mat().inverse()
    }

    pub fn frustum_planes(&self) -> [Vec4; 6] {
        let frustum = self.frustum();
        let left_plane = frustum.row(3) + frustum.row(0);
        let right_plane = frustum.row(3) - frustum.row(0);
        let bottom_plane = frustum.row(3) + frustum.row(1);
        let top_plane = frustum.row(3) - frustum.row(1);
        let near_plane = frustum.row(3) + frustum.row(2);
        let far_plane = frustum.row(3) - frustum.row(2);
        [
            left_plane,
            right_plane,
            bottom_plane,
            top_plane,
            near_plane,
            far_plane,
        ]
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            game_object: Rc::new(RefCell::new(BaseGameObject {
                data: GameObjectData::new(None),
                components: ComponentMap::new(),
            }))
        }
    }
}