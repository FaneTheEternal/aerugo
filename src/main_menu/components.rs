use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum MainMenuButtons {
    NewGame,
    Load,
    Settings,
    Exit,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Component)]
pub struct MainMenuButton {
    pub target: MainMenuButtons,
}
