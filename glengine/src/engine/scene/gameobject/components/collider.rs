use rapier3d::prelude::*;
use glengine_mdl::models::MeshStruct;
use crate::engine::scene::gameobject::components::{Component};

pub struct ColliderComponent {
    collider: Collider
}

impl From<Collider> for ColliderComponent {
    fn from(collider: Collider) -> Self {
        Self{collider}
    }
}

impl From<&MeshStruct> for ColliderComponent {
    fn from(mesh: &MeshStruct) -> Self {
        let vertices: Vec<Point<f32>> = mesh.vertices.chunks(4).map(|v|{
            Point::new(v[0],v[1],v[2])
        }).collect();
        let indices: Vec<[u32;3]> = mesh.indices.chunks(3).map(|v|{
            [v[0],v[1],v[2]]
        }).collect();
        let collider = ColliderBuilder::trimesh(vertices,indices).build();
        Self{collider}
    }
}

impl Component for ColliderComponent {

}
