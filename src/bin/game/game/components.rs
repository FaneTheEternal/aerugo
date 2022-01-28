use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameButtons {
    Back,
    Forward,
    Menu,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Component)]
pub struct GameButton {
    pub target: GameButtons,
}

#[derive(Debug, Clone, Component)]
pub struct SpriteMark {
    pub name: String,
    pub timer: Timer,
    pub is_await: bool,
    pub is_rev: bool,
}

impl SpriteMark {
    pub fn new(name: &str) -> SpriteMark {
        SpriteMark {
            name: name.to_string(),
            timer: Timer::from_seconds(1.0, false),
            is_await: true,
            is_rev: false,
        }
    }
}
