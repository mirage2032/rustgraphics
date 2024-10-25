use glam::{Quat, Vec3};
use rapier3d::prelude::*;
use crate::engine::scene::gameobject::base::GameObjectData;
use crate::engine::scene::gameobject::components::{Component, ComponentMap};
use crate::engine::transform::Transform;

pub struct RigidBodyComponent {
    pub rigid_body: RigidBody
}

impl RigidBodyComponent {
    pub fn set_transform(&mut self, transform:&Transform) {
        self.rigid_body.set_translation(transform.position.into(),false);
        self.rigid_body.set_rotation(transform.rotation.into(),false);
    }
    
    pub fn get_rotation(&self)->Quat{
        self.rigid_body.rotation().quaternion().clone().into()
    }
    
    pub fn get_position(&self)->Vec3{
        self.rigid_body.translation().clone().into()
    }
}

impl From<RigidBody> for RigidBodyComponent {
    fn from(rigid_body: RigidBody) -> Self {
        Self{rigid_body}
    }
}

impl Component for RigidBodyComponent {
    fn setup(&mut self, object: &mut GameObjectData, _components: &ComponentMap) {
        self.set_transform(&object.transform);
    }
}
