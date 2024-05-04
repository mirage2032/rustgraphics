use glam::Mat4;

use crate::engine::drawable::Drawable;
use crate::engine::transform::Transform;

pub trait GameObject<'a>: Drawable {
    fn data(&self) -> &GameObjectData<'a>;
    fn data_mut(&mut self) -> &mut GameObjectData<'a>;
    fn step(&mut self);
}

impl<'a, T: GameObject<'a>> Drawable for T {
    fn draw(&self, modelmat: &Mat4, viewmat: &Mat4) {
        let data = self.data();
        let newmodelmat = *modelmat * data.transform.to_mat4();
        if let Some(drawable) = &data.drawable {
            drawable.draw(&newmodelmat, viewmat);
        }
        for child in &data.children {
            child.draw(&newmodelmat, viewmat);
        }
    }
}


pub struct GameObjectData<'a> {
    parent: Option<&'a dyn GameObject<'a>>,
    children: Vec<Box<dyn GameObject<'a>>>,
    transform: Transform,
    drawable: Option<Box<dyn Drawable>>,
}

impl<'a> GameObjectData<'a> {
    pub fn new(parent: Option<&'a dyn GameObject<'a>>) -> Self {
        Self {
            parent,
            children: Vec::new(),
            transform: Transform::default(),
            drawable: None,
        }
    }
}