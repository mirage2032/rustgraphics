use std::any::Any;

use crate::engine::drawable::Drawable;
use crate::engine::GameState;
use crate::engine::scene::gameobject::components::{Component, ComponentMap};
use crate::engine::scene::gameobject::GameObjectData;
use crate::result::EngineStepResult;

pub struct DrawableComponent {
    pub drawable: Box<dyn Drawable>,
}

impl DrawableComponent {
    pub fn new(drawable: Box<dyn Drawable>) -> Self {
        Self { drawable }
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
