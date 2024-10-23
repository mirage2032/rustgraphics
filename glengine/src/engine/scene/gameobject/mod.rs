use std::cell::RefCell;
use std::rc::Rc;
use crate::engine::transform::Transform;
use crate::engine::scene::gameobject::base::BaseGameObject;

pub mod base;
pub mod components;

pub type GameObject = Rc<RefCell<BaseGameObject>>;

pub struct GameObjectData {
    pub parent: Option<GameObject>,
    pub children: Vec<GameObject>,
    pub transform: Transform,
}

impl GameObjectData {
    pub fn new(parent: Option<GameObject>) -> Self {
        Self {
            parent,
            children: Vec::new(),
            transform: Transform::default(),
        }
    }
    
    pub fn new_w_transform(parent:Option<GameObject>,transform:Transform)->Self{
        Self{
            parent,
            children:Vec::new(),
            transform
        }
    }
}
