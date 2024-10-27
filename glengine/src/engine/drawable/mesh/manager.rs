use std::collections::HashMap;
use std::rc::{Rc, Weak};
use crate::engine::drawable::mesh::Mesh;

#[derive(Clone)]
pub struct MeshHandle{
    rc: Rc<()>,
    handle:usize
}

#[derive(Clone)]
struct MeshWeakHandle{
    handle: Weak<()>
}

pub struct MeshManager {
    meshes: HashMap<usize, (Box<dyn Mesh>,MeshWeakHandle)>,
    index: usize
}

impl MeshManager {
    pub fn get(&self, handle: &MeshHandle) -> Option<&Box<dyn Mesh>>{
        let (mesh,_) = self.meshes.get(&handle.handle)?;
        Some(mesh)
    }
    pub fn get_mut(&mut self, handle: &MeshHandle) -> Option<&mut Box<dyn Mesh>>{
        let (mesh,_) = self.meshes.get_mut(&handle.handle)?;
        Some(mesh)
    }
    pub fn add(&mut self, mesh: Box<dyn Mesh>) -> MeshHandle{
        let index = self.index;
        let mesh_handle = MeshHandle{
            rc: Rc::new(()),
            handle: index
        };
        let weak_handle = MeshWeakHandle{
            handle: Rc::downgrade(&mesh_handle.rc)
        };
        self.meshes.insert(index, (mesh,weak_handle));

        self.index = self.index.wrapping_add(1);
        while let Some((_,weak)) = self.meshes.get(&self.index){
            match weak.handle.upgrade(){
                Some(_) => self.index = self.index.wrapping_add(1),
                None => {
                    self.meshes.remove(&self.index);
                    break;
                }
            }
        }
        mesh_handle
    }

    pub fn clean(&mut self){
        self.meshes.retain(|_,(_,weak)|weak.handle.upgrade().is_some());
    }
    pub fn remove(&mut self, handle: MeshHandle){
        self.meshes.remove(&handle.handle);
    }
}

impl Default for MeshManager {
    fn default() -> Self{
        Self{
            meshes: HashMap::new(),
            index: 0
        }
    }
}