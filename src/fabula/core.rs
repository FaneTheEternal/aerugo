#![allow(dead_code)]

use crate::types::utility::GameState;

pub struct CoreMachine {
}

impl CoreMachine {
    pub fn new() -> CoreMachine {
        CoreMachine {}
    }

    pub fn extract_state(&self) -> GameState {
        GameState::NOP
    }
}