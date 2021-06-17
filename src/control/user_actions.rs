#![allow(dead_code)]

use sdl2::mouse::MouseButton;
use std::collections::hash_set::IntoIter;
use std::collections::HashSet;

use sdl2::keyboard::Keycode;

#[derive(Clone)]
pub struct UserActions {
    keyboard: HashSet<Keycode>,
    mouse: UserMouse,
}

unsafe impl Send for UserActions {}

impl UserActions {
    pub fn new() -> UserActions {
        let keyboard = HashSet::new();
        let mouse = UserMouse::new();
        UserActions {keyboard, mouse}
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

    pub fn push_mouse(&mut self, button: MouseButton) {
        self.mouse.click(button);
    }

    pub fn release_mouse(&mut self, button: MouseButton) {
        self.mouse.release(button);
    }

    pub fn move_mouse(&mut self, x: i32, y: i32) {
        self.mouse.r#move(x, y);
    }
}


#[derive(Clone)]
pub struct UserMouse {
    x: i32,
    y: i32,
    buttons: HashSet<MouseButton>,
}

unsafe impl Send for UserMouse {}

impl UserMouse {
    pub fn new() -> UserMouse {
        UserMouse{
            x: -1,
            y: -1,
            buttons: HashSet::new()
        }
    }

    pub fn click(&mut self, button: MouseButton) {
        self.buttons.insert(button);
    }

    pub fn release(&mut self, button: MouseButton) {
        self.buttons.remove(&button);
    }

    pub fn r#move(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn left(&self) -> bool {
        self.buttons.contains(&MouseButton::Left)
    }
    
    pub fn right(&self) -> bool {
        self.buttons.contains(&MouseButton::Right)
    }
}