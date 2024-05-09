use std::collections::HashSet;
use glfw::{Key,Action,MouseButton};

pub enum EngineWindowEvent {
    Close,
    Resize(u32,u32),
}

pub trait MouseOrKeyboardKey: Eq + std::hash::Hash + Copy
{}

impl MouseOrKeyboardKey for Key{}
impl MouseOrKeyboardKey for MouseButton{}

pub struct KeyState<T:MouseOrKeyboardKey> {
    pub pressed_keys: HashSet<T>,
    pub released_keys: HashSet<T>,
    pub repeated_keys: HashSet<T>,
    pub held_keys: HashSet<T>,
}

impl<T:MouseOrKeyboardKey> KeyState<T> {
    pub fn new() -> Self {
        Self {
            pressed_keys: HashSet::new(),
            released_keys: HashSet::new(),
            repeated_keys: HashSet::new(),
            held_keys: HashSet::new(),
        }
    }
    pub fn add_key(&mut self,key: T,action: Action) {
        match action {
            Action::Press => {
                self.pressed_keys.insert(key);
            },
            Action::Release => {
                self.released_keys.insert(key);
            },
            Action::Repeat => {
                self.repeated_keys.insert(key);
            },
        }
    }
    
    pub fn is_pressed(&self,key: T) -> bool {
        self.pressed_keys.contains(&key)
    }
    
    pub fn is_held(&self,key: T) -> bool {
        self.held_keys.contains(&key)
    }
    
    pub fn is_released(&self,key: T) -> bool {
        self.released_keys.contains(&key)
    }
    
    pub fn merge(&mut self,other: Self) {
        self.pressed_keys=other.pressed_keys;
        self.released_keys=other.released_keys;
        self.repeated_keys=other.repeated_keys;
        for key in self.pressed_keys.iter() {
            self.held_keys.insert(*key);
        }
        for key in self.pressed_keys.iter() {
            self.held_keys.insert(*key);
        }
        for key in self.released_keys.iter() {
            self.held_keys.remove(key);
        }
    }
}


pub struct EngineKeyState {
    //hashmap key to action
    pub keyboard: KeyState<Key>,
    pub mouse: KeyState<MouseButton>,
}

impl EngineKeyState {
    pub fn new() -> Self {
        Self {
            keyboard: KeyState::new(),
            mouse: KeyState::new(),
        }
    }
    
    pub fn merge(&mut self,other: Self) {
        self.keyboard.merge(other.keyboard);
        self.mouse.merge(other.mouse);
    }
}



