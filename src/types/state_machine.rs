#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::types::utility::GameState;

pub struct SimpleCore {
    pub counter: u32,
}

impl SimpleCore {
    pub fn new() -> Rc<RefCell<SimpleCore>> {
        Rc::new(RefCell::new(SimpleCore {
            counter: 0
        }))
    }


    pub fn extract_state(&self) -> GameState {
        GameState::NOP
    }
}