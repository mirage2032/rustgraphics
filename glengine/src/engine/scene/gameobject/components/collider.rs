use rapier3d::prelude::*;
use glengine_mdl::models::MeshStruct;
use crate::engine::scene::gameobject::components::{Component};

fn get_mesh_vertices(mesh:&MeshStruct,scale:f32) -> Vec<Point<f32>>{
    mesh.vertices.chunks(3).map(|v|{
        Point::new(v[0]*scale,v[1]*scale,v[2]*scale)
    }).collect()
}

fn get_mesh_indices(mesh:&MeshStruct) -> Vec<[u32;3]>{
    mesh.indices.chunks(3).map(|v|{
        [v[0],v[1],v[2]]
    }).collect()
}

pub struct ColliderComponent {
    pub colliders: Vec<Collider>
}

impl ColliderComponent {
    pub fn trimesh_from_mesh(mesh: &MeshStruct,scale:f32) -> Self {
        let vertices = get_mesh_vertices(mesh,scale);
        let indices: Vec<[u32;3]> = get_mesh_indices(mesh);
        let collider = ColliderBuilder::trimesh(vertices,indices).build();
        Self{ colliders:vec![collider]}
    }
    
    pub fn hull_from_mesh(mesh: &MeshStruct,scale:f32) -> Self {
        let vertices = get_mesh_vertices(mesh,scale);
        let collider = ColliderBuilder::convex_hull(&vertices).unwrap().build();
        Self{ colliders:vec![collider]}
    }
    
    pub fn trimesh_from_meshvec(meshes: &Vec<MeshStruct>,scale:f32) -> Self {
        let mut colliders = vec![];
        for mesh in meshes {
            let vertices = get_mesh_vertices(mesh,scale);
            let indices: Vec<[u32;3]> = get_mesh_indices(mesh);
            let collider = ColliderBuilder::trimesh(vertices,indices).build();
            colliders.push(collider);
        }
        Self{ colliders }
    }
    
    pub fn hull_from_meshvec(meshes: &Vec<MeshStruct>,scale:f32) -> Self {
        let mut colliders = vec![];
        for mesh in meshes {
            let vertices = get_mesh_vertices(mesh,scale);
            let collider = ColliderBuilder::convex_hull(&vertices).unwrap().build();
            colliders.push(collider);
        }
        Self{ colliders }
    }
}

impl From<Vec<Collider>> for ColliderComponent {
    fn from(collider: Vec<Collider>) -> Self {
        Self{ colliders: collider }
    }
}

impl From<Collider> for ColliderComponent {
    fn from(collider: Collider) -> Self {
        Self{ colliders: vec![collider]}
    }
}

impl Component for ColliderComponent {

}
