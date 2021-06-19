#![allow(dead_code)]

use sdl2::mouse::MouseButton;
use std::collections::hash_set::IntoIter;
use std::collections::{HashSet, HashMap};

use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

#[derive(Clone)]
pub struct UserActions {
    keyboard: HashSet<Keycode>,
    pub mouse: UserMouse,
}

unsafe impl Send for UserActions {}

impl UserActions {
    pub fn new() -> UserActions {
        let keyboard = HashSet::new();
        let mouse = UserMouse::new();
        UserActions {keyboard, mouse}
    }

    pub fn tick(&mut self) {
        self.mouse.tick()
    }

    pub fn push_key(&mut self, key: Keycode) {
        self.keyboard.insert(key);
    }

    pub fn release_key(&mut self, key: Keycode) {
        self.keyboard.remove(&key);
    }

    pub fn dump_keys(&self) -> IntoIter<Keycode> {
        self.keyboard.clone().into_iter()
    }
}


#[derive(Clone)]
pub struct UserMouse {
    x: i32,
    y: i32,
    buttons: HashMap<MouseButton, (i32, i32)>,
    released: HashMap<MouseButton, ((i32, i32), (i32, i32))>,
}

unsafe impl Send for UserMouse {}

impl UserMouse {
    pub fn new() -> UserMouse {
        UserMouse{
            x: -1,
            y: -1,
            buttons: Default::default(),
            released: Default::default()
        }
    }

    pub fn click(&mut self, button: MouseButton) {
        self.buttons.insert(button, (self.x, self.y));
    }

    pub fn release(&mut self, button: MouseButton) {
        let start = self.buttons.remove(&button).unwrap();
        let end = (self.x, self.y);
        self.released.insert(button, (start, end));
    }

    pub fn tick(&mut self) {
        self.released.clear()
    }

    pub fn r#move(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn is_in(&self, rect: Rect) -> bool {
        rect.contains_point((self.x, self.y))
    }

    pub fn is_pushed(&self, button: MouseButton) -> Option<(i32, i32)> {
         self.buttons.get(&button).cloned()
    }

    pub fn is_released(&self, button: MouseButton) -> Option<((i32, i32), (i32, i32))> {
         self.released.get(&button).cloned()
    }
}