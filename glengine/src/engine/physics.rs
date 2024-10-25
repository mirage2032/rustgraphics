use glam::{Quat, Vec3};
use rapier3d::na::Vector3;
use rapier3d::prelude::*;
use crate::engine::config::CONFIG;
use crate::engine::scene::gameobject::components::collider::ColliderComponent;
use crate::engine::scene::gameobject::components::rigidbody::RigidBodyComponent;
use crate::engine::scene::gameobject::GameObject;

pub struct PhysicsData {
    gravity: Vector3<f32>,
    physics_pipeline: PhysicsPipeline,
    integration_parameters: IntegrationParameters,
    island_manager: IslandManager,
    // broad_phase: BroadPhaseMultiSap,
    // narrow_phase: NarrowPhase,
    // rigid_body_set: RigidBodySet,
    // collider_set: ColliderSet,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    query_pipeline: QueryPipeline,
}

impl PhysicsData {
    pub fn step(&mut self, game_objects: &Vec<GameObject>) {
        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();
        let mut broad_phase = BroadPhaseMultiSap::new();
        let mut narrow_phase = NarrowPhase::new();
        let mapped = game_objects.iter().map(|obj|{
            let rigid_body_comp = obj.base.borrow().components.get_component::<RigidBodyComponent>().expect("RigidBodyComponent not found");
            let collider_comp = obj.base.borrow().components.get_component::<ColliderComponent>().expect("ColliderComponent not found");
            let rigid_body_handle = rigid_body_set.insert(rigid_body_comp.borrow().rigid_body.clone());
            for collider in collider_comp.borrow().colliders.iter(){
                collider_set.insert_with_parent(collider.clone(), rigid_body_handle, &mut rigid_body_set);
            }
            (rigid_body_handle, obj)
        }).collect::<Vec<_>>();
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut broad_phase,
            &mut narrow_phase,
            &mut rigid_body_set,
            &mut collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            &(),
            &()
        );
        for (rigid_body_handle, obj) in mapped{
            let mut obj = obj.base.borrow_mut();
            let rigid_body = &rigid_body_set[rigid_body_handle];
            let comp = obj.components.get_component::<RigidBodyComponent>().unwrap();
            let mut borrowed_comp = comp.borrow_mut();
            borrowed_comp.rigid_body = rigid_body.clone();
            obj.data.transform.position = borrowed_comp.get_position();
            obj.data.transform.rotation = borrowed_comp.get_rotation();
        }
    }
}


impl Default for PhysicsData{
    fn default()->Self{
        let integration_parameters = IntegrationParameters{
            dt: CONFIG.config().get_fixed_step().as_millis() as f32/1000.0,
            ..IntegrationParameters::default()
        };
        Self{
            gravity: Vector3::new(0.0, -9.81, 0.0),
            physics_pipeline: PhysicsPipeline::new(),
            integration_parameters,
            island_manager: IslandManager::new(),
            // broad_phase: BroadPhaseMultiSap::new(),
            // narrow_phase: NarrowPhase::new(),
            // rigid_body_set: RigidBodySet::new(),
            // collider_set: ColliderSet::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            query_pipeline: QueryPipeline::new(),
        }
    }
}