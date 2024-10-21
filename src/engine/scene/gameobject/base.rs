use std::cell::RefCell;
use std::rc::Rc;

use crate::engine::scene::gameobject::components::ComponentMap;
use crate::engine::scene::gameobject::{GameObject, GameObjectData, GameObjectTrait};
use crate::engine::GameState;
use crate::result::EngineStepResult;

pub struct BaseGameObject {
    data: GameObjectData,
    components: ComponentMap,
}

impl BaseGameObject {
    pub fn new(parent: Option<GameObject>) -> GameObject {
        let newgameobject = Rc::new(RefCell::new(Self {
            data: GameObjectData::new(parent.clone()),
            components: ComponentMap::new(),
        }));
        if let Some(parent) = parent {
            parent
                .borrow_mut()
                .data_mut()
                .children
                .push(newgameobject.clone());
        }
        newgameobject
    }
}

impl GameObjectTrait for BaseGameObject {
    fn data(&self) -> &GameObjectData {
        &self.data
    }

    fn data_mut(&mut self) -> &mut GameObjectData {
        &mut self.data
    }
    
    fn components(&self) -> Option<&ComponentMap> {
        Some(&self.components)
    }
    
    fn components_mut(&mut self) -> Option<&mut ComponentMap> {
        Some(&mut self.components)
    }
    
    fn step(&mut self, state: &GameState) -> EngineStepResult<()> {
        self.components.step(&mut self.data, state)?;
        Ok(())
    }
}
