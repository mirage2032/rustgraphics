use std::sync::{Arc, Mutex};

use glam::{Mat4, Vec3};

use crate::engine::drawable::Draw;
use crate::engine::GameState;
use crate::engine::scene::lights::Lights;
use crate::engine::transform::Transform;
use crate::result::EngineStepResult;

pub trait GameObjectRaw: Draw + Send {
    fn data(&self) -> &GameObjectData;
    fn data_mut(&mut self) -> &mut GameObjectData;
    fn step(&mut self, _: &GameState) -> EngineStepResult<()>{ Ok(()) }
    fn step_recursive(&mut self, game: &GameState) -> EngineStepResult<()> {
        self.step(game)?;
        for child in &mut self.data_mut().children {
            child
                .lock()
                .expect("Could not lock child gameobject for step")
                .step(game)?;
        }
        Ok(())
    }
    fn global_mat(&self) -> Mat4 {
        let mut transform: Mat4 = self.data().transform.into();
        let mut parent = self.data().parent.clone();
        while let Some(parent_object) = parent {
            let parent_data = parent_object
                .lock()
                .expect("Could not lock parent gameobject for global transform");
            transform = Mat4::from(parent_data.data().transform) * transform;
            parent = parent_data.data().parent.clone();
        }
        transform
    }
    
    fn glob_pos(&self) -> Vec3 {
        let mut position = self.data().transform.position;
        let mut parent = self.data().parent.clone();
        while let Some(parent_object) = parent {
            let parent_data = parent_object
                .lock()
                .expect("Could not lock parent gameobject for global transform");
            position = parent_data.data().transform.position + position;
            parent = parent_data.data().parent.clone();
        }
        position
    }
}

pub type GameObject = Arc<Mutex<dyn GameObjectRaw>>;

impl<T: GameObjectRaw> Draw for T {
    fn draw(&self, modelmat: &Mat4, viewmat: &Mat4, lights: &Lights) {
        let data = self.data();
        let newmodelmat = *modelmat * Mat4::from(data.transform);
        if let Some(drawable) = &data.drawable {
            drawable.draw(&newmodelmat, viewmat,lights);
        }
        for child in &data.children {
            child
                .lock()
                .expect("Could not lock child gameobject for draw")
                .draw(&newmodelmat, viewmat,lights);
        }
    }
}

pub struct GameObjectData {
    pub parent: Option<GameObject>,
    pub children: Vec<GameObject>,
    pub transform: Transform,
    pub drawable: Option<Box<dyn Draw>>,
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
        let newgameobject = Arc::new(Mutex::new(Self {
            data: GameObjectData::new(parent.clone()),
        }));
        if let Some(parent) = parent {
            parent
                .lock()
                .expect("Could not lock parent gameobject for init")
                .data_mut()
                .children
                .push(newgameobject.clone());
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

    fn step(&mut self, _game: &GameState) -> EngineStepResult<()> {
        Ok(())
    }
}

pub struct RotatingGameObject {
    data: GameObjectData,
    rotation: Vec3,
}

impl RotatingGameObject {
    pub fn new(parent: Option<GameObject>, rotation: Vec3) -> GameObject {
        let newgameobject = Arc::new(Mutex::new(Self {
            data: GameObjectData::new(parent.clone()),
            rotation,
        }));
        if let Some(parent) = parent {
            parent
                .lock()
                .expect("Could not lock parent gameobject for init")
                .data_mut()
                .children
                .push(newgameobject.clone());
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

    fn step(&mut self, game: &GameState) -> EngineStepResult<()> {
        let duration = game.delta.as_secs_f32();
        let rotation = self.rotation * duration;
        let data = self.data_mut();
        data.transform.rotation *= glam::Quat::from_rotation_x(rotation.x);
        data.transform.rotation *= glam::Quat::from_rotation_y(rotation.y);
        data.transform.rotation *= glam::Quat::from_rotation_z(rotation.z);
        Ok(())
    }
}
