use std::any::Any;
use glam::Mat4;

use crate::engine::drawable::Drawable;
use crate::engine::scene::gameobject::components::Component;
use crate::engine::scene::lights::Lights;

pub struct DrawableComponent {
    pub drawable: Box<dyn Drawable>,
}

impl DrawableComponent {
    pub fn new(drawable: Box<dyn Drawable>) -> Self {
        Self { drawable }
    }
}

impl Drawable for DrawableComponent {
    fn draw(&self,modelmat: &Mat4, viewmat: &Mat4, lights: Option<&Lights>) {
        self.drawable.draw(modelmat, viewmat, lights);
    }
}

impl Component for DrawableComponent {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
