use glam::Mat4;
use crate::engine::drawable::Drawable;
use crate::engine::transform::Transform;

trait Step {
    fn step(&mut self);
}

pub struct GameObject<'a> {
    parent: Option<&'a GameObject<'a>>,
    children: Vec<GameObject<'a>>,
    transform: Transform,
    drawable: Option<Box<dyn Drawable>>,
}

impl<'a> GameObject<'a> {
    pub fn new(parent: Option<&'a GameObject>) -> Self {
        Self {
            parent,
            children: Vec::new(),
            transform: Transform::default(),
            drawable: None,
        }
    }
}

impl<'a> Step for GameObject<'a> {
    fn step(&mut self) {
        for child in &mut self.children {
            child.step();
        }
    }
}
impl Drawable for GameObject<'_>{
    fn draw(&self,modelmat: &Mat4, viewmat: &Mat4) {
        if let Some(drawable) = &self.drawable {
            drawable.draw(modelmat, viewmat);
        }
        for child in &self.children {
            child.draw(modelmat, viewmat);
        }
    }

}