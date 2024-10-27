use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use once_cell::unsync::Lazy;
use crate::engine::drawable::material::Material;

#[derive(Clone)]
pub struct MaterialHandle {
    rc: Rc<()>,
    handle: usize,
}
pub struct MaterialWeakHandle {
    handle: Weak<()>,
}

pub struct MaterialManager {
    materials: HashMap<usize, (Material,MaterialWeakHandle)>,
    index: usize,
}


impl MaterialManager {
    pub fn get(&self, handle:&MaterialHandle) -> Option<&Material> {
        self.materials.get(&handle.handle).map(|(material,_)| material)
    }

    pub fn get_mut(&mut self, handle:&MaterialHandle) -> Option<&mut Material> {
        self.materials.get_mut(&handle.handle).map(|(material,_)| material)
    }
    pub fn add(&mut self, material: Material) -> MaterialHandle {
        let index = self.index;
        let handle = MaterialHandle {
            rc: Rc::new(()),
            handle: index,
        };
        let weak_handle = MaterialWeakHandle {
            handle: Rc::downgrade(&handle.rc),
        };
        self.materials.insert(index, (material,weak_handle));
        self.index += 1;
        while let Some((_,weak)) = self.materials.get(&self.index){
            match weak.handle.upgrade(){
                Some(_) => self.index += 1,
                None => {
                    self.materials.remove(&self.index);
                    break;
                }
            }
        }
        handle
    }

    pub fn clean(&mut self) {
        self.materials.retain(|_,(_,weak)|weak.handle.upgrade().is_some());
    }

    pub fn remove(&mut self, id: usize) {
        self.materials.remove(&id);
    }

}

impl Default for MaterialManager {
    fn default() -> Self {
        Self {
            materials: HashMap::new(),
            index: 0,
        }
    }
}