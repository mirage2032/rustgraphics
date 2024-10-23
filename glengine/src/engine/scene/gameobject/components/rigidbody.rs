use glam::{vec3, Quat, Vec3};
use rapier3d::na::UnitQuaternion;
use rapier3d::prelude::*;
use crate::engine::GameState;
use crate::engine::scene::gameobject::components::{Component, ComponentMap};
use crate::engine::scene::gameobject::GameObjectData;
use crate::engine::transform::Transform;
use crate::result::EngineStepResult;

pub struct RigidBodyComponent {
    fixed:bool,
    pub rigid_body: rapier3d::dynamics::RigidBody
}

impl RigidBodyComponent {
    pub fn new(fixed:bool) -> Self {
        let rigid_body = match fixed {
            true =>RigidBodyBuilder::fixed().build(),
            false =>RigidBodyBuilder::dynamic().build()
        };
        Self {fixed,rigid_body}
    }
    pub fn fixed(&self)->bool{
        self.fixed
    }
    pub fn set_fixed(&mut self,fixed:bool){
        self.fixed = fixed;
        let translation = self.rigid_body.translation();
        self.rigid_body = match fixed {
            true =>RigidBodyBuilder::fixed().translation(translation.clone()).build(),
            false =>RigidBodyBuilder::dynamic().translation(translation.clone()).build()
        };
    }

    pub fn set_transform(&mut self, transform:&Transform) {
        self.rigid_body.set_translation(vector!(transform.position.x,transform.position.y,transform.position.z),false);
        let rot = transform.rotation.to_array();
        self.rigid_body.set_rotation(UnitQuaternion::from_quaternion(nalgebra::Quaternion::new(rot[0],rot[1],rot[2],rot[3])),false);
    }
    
    pub fn get_rotation(&self)->Quat{
        let rot = self.rigid_body.rotation().quaternion();
        Quat::from_array([rot[0],rot[1],rot[2],rot[3]])
    }
    
    pub fn get_position(&self)->Vec3{
        let pos = self.rigid_body.translation();
        vec3(pos.x,pos.y,pos.z)
    }
}

impl Component for RigidBodyComponent {
    fn setup(&mut self, object: &mut GameObjectData, _components: &ComponentMap) {
        self.set_transform(&object.transform);
    }
}
