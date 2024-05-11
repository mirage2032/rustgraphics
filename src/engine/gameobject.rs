use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;

use glam::{Mat4, Vec3};
use glfw::Key;

use crate::engine::drawable::Drawable;
use crate::engine::GameState;
use crate::engine::transform::Transform;

pub trait GameObjectRaw: Drawable {
    fn data(&self) -> &GameObjectData;
    fn data_mut(&mut self) -> &mut GameObjectData;
    fn step(&mut self, game:&GameState);
    fn step_recursive(&mut self, game:&GameState) {
        self.step(game);
        for child in &mut self.data_mut().children {
            child
                .write()
                .expect("Could not lock child gameobject for step")
                .step(game);
        }
    }
}

pub type GameObject = Arc<RwLock<dyn GameObjectRaw>>;

impl<T: GameObjectRaw> Drawable for T {
    fn draw(&self, modelmat: &Mat4, viewmat: &Mat4) {
        let data = self.data();
        let newmodelmat = *modelmat * Mat4::from(data.transform);
        if let Some(drawable) = &data.drawable {
            drawable.draw(&newmodelmat, viewmat);
        }
        for child in &data.children {
            child
                .read()
                .expect("Could not lock child gameobject for draw")
                .draw(&newmodelmat, viewmat);
        }
    }
}

pub struct GameObjectData {
    pub parent: Option<GameObject>,
    pub children: Vec<GameObject>,
    pub transform: Transform,
    pub drawable: Option<Box<dyn Drawable>>,
}

impl GameObjectData {
    pub fn new(parent: Option<GameObject>) -> Self {
        Self {
            parent,
            children: Vec::new(),
            transform: Transform::default(),
            drawable: None,
        }
    }
}

pub struct BaseGameObject {
    data: GameObjectData,
}

impl BaseGameObject {
    pub fn new(parent: Option<GameObject>) -> GameObject {
        let newgameobject = Arc::new(RwLock::new(Self {
            data: GameObjectData::new(parent.clone()),
        }));
        if let Some(parent) = parent {
            parent.write().expect("Could not lock parent gameobject for init").data_mut().children.push(newgameobject.clone());
        }
        newgameobject
    }
}

impl GameObjectRaw for BaseGameObject {
    fn data(&self) -> &GameObjectData {
        &self.data
    }

    fn data_mut(&mut self) -> &mut GameObjectData {
        &mut self.data
    }

    fn step(&mut self, _game: &GameState) {
    }
}

pub struct RotatingGameObject {
    data: GameObjectData,
    rotation: Vec3,
}

impl RotatingGameObject {
    pub fn new(parent: Option<GameObject>, rotation: Vec3) -> GameObject {
        let newgameobject = Arc::new(RwLock::new(Self {
            data: GameObjectData::new(parent.clone()),
            rotation,
        }));
        if let Some(parent) = parent {
            parent.write().expect("Could not lock parent gameobject for init").data_mut().children.push(newgameobject.clone());
        }
        newgameobject
    }
}

impl GameObjectRaw for RotatingGameObject {
    fn data(&self) -> &GameObjectData {
        &self.data
    }

    fn data_mut(&mut self) -> &mut GameObjectData {
        &mut self.data
    }

    fn step(&mut self, game: &GameState) {
        let duration = game.delta.as_secs_f32();
        let rotation = self.rotation * duration;
        let data = self.data_mut();
        data.transform.rotation *= glam::Quat::from_rotation_x(rotation.x);
        data.transform.rotation *= glam::Quat::from_rotation_y(rotation.y);
        data.transform.rotation *= glam::Quat::from_rotation_z(rotation.z);
    }
}
