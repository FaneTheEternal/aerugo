use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum OverlayButtons {
    Close,
    Settings,
    Save,
    Load,
    MainMenu,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Component)]
pub struct OverlayButton {
    pub target: OverlayButtons,
}
