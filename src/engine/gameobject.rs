use nalgebra_glm::TMat4;
use crate::engine::drawable::Drawable;
use crate::engine::transform::Transform;

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

impl Drawable for GameObject<'_>{
    fn draw(&self,modelmat: &TMat4<f32>, viewmat: &TMat4<f32>) {
        if let Some(drawable) = &self.drawable {
            drawable.draw(modelmat, viewmat);
        }
        for child in &self.children {
            child.draw(modelmat, viewmat);
        }
    }

}