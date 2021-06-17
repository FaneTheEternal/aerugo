#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;
use std::fmt::Formatter;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use crate::types::utility::GameState;

pub struct SimpleCore {
    pub x: u32,
    pub y: u32,
    pub color: Color,

    is_ini: bool,
}

impl std::fmt::Display for SimpleCore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[x({}) y({})]", self.x, self.y)
    }
}

impl SimpleCore {
    pub fn new() -> Rc<RefCell<SimpleCore>> {
        Rc::new(RefCell::new(SimpleCore {
            x: 0,
            y: 0,
            color: Color::MAGENTA,
            is_ini: false,
        }))
    }

    pub fn ini(&mut self, _bound: Rect) {
        if !self.is_ini {
            self.is_ini = true;
        }
    }

    pub fn re_color(&mut self, num: u8) {
        self.color = match num {
            1 => { Color::MAGENTA }
            2 => { Color::WHITE }
            3 => { Color::BLACK }
            4 => { Color::GRAY }
            5 => { Color::GREEN }
            6 => { Color::BLUE }
            7 => { Color::RED }
            8 => { Color::YELLOW }
            9 => { Color::CYAN }
            _ => { Color::MAGENTA }
        }
    }

    pub fn up(&mut self, _bound: Rect) {
        if self.y > 0 {
            self.y -= 1;
        }
    }

    pub fn down(&mut self, bound: Rect) {
        if self.y < bound.height() {
            self.y += 1;
        }
    }

    pub fn left(&mut self, _bound: Rect) {
        if self.x > 0 as u32 {
            self.x -= 1;
        }
    }

    pub fn right(&mut self, bound: Rect) {
        if self.x < bound.width() {
            self.x += 1;
        }
    }

    pub fn extract_state(&self) -> GameState {
        GameState::NOP
    }
}