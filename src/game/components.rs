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
