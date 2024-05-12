use std::collections::HashMap;
use std::any::{TypeId,Any};
use std::cell::RefCell;

pub trait Component : Any {}

pub struct CompA{
    pub a: i32,
}
impl Component for CompA{}

pub struct CompB{
    pub a: i32,
}
impl Component for CompB {}

pub struct ComponentMap {
    elements: HashMap<TypeId, RefCell<Box<dyn Component>>>,
}

impl ComponentMap {
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
        }
    }

    pub fn add_component<T: 'static + Component>(&mut self, component: T) {
        let type_id = TypeId::of::<T>();
        self.elements.insert(type_id, RefCell::new(Box::new(component)));
    }

    pub fn get_component<T: 'static + Component>(&self) -> Option<&RefCell<Box<T>>> {
        let type_id = TypeId::of::<T>();
        match self.elements.get(&type_id) {
            Some(component) => {
                unsafe {
                    let component = std::mem::transmute::<&RefCell<Box<dyn Component>>, &RefCell<Box<T>>>(component);
                    Some(component)
                }
            }
            None => None,
        }
    }
}
