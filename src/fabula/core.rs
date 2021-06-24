#![allow(dead_code)]

use crate::types::utility::GameState;

#[derive(Copy, Clone)]
pub enum Replies {
    Yo,
    Ohayo,
    Kawaii,
}

impl Replies {
    pub fn verbose(&self) -> String {
        String::from(match *self {
            Replies::Yo => { "Yo" }
            Replies::Ohayo => { "Ohayo" }
            Replies::Kawaii => { "Kawaii" }
        })
    }

    pub fn next(&self) -> Replies {
        match *self {
            Replies::Yo => { Replies::Ohayo }
            Replies::Ohayo => { Replies::Kawaii }
            Replies::Kawaii => { Replies::Yo }
        }
    }
}

pub struct CoreMachine {
    pub replica: Replies,
    pub in_choices: bool,
}

impl CoreMachine {
    pub fn new() -> CoreMachine {
        CoreMachine { replica: Replies::Yo, in_choices: false }
    }

    pub fn wanna_choice(&mut self) {
        self.in_choices = true;
    }

    pub fn next(&mut self) {
        self.replica = self.replica.next();
    }

    pub fn choice(&mut self, replica: Replies) {
        self.replica = replica;
        self.in_choices = false;
    }

    pub fn verbose(&self) -> String {
        self.replica.verbose()
    }

    pub fn extract_state(&self) -> GameState {
        GameState::NOP
    }
}