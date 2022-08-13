use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use serde::{Deserialize, Serialize};

#[derive(Component, Reflect, Default, Clone, Inspectable, Serialize, Deserialize)]
#[reflect(Component)]
pub enum MainMenuButtons {
    #[default]
    NewGame,
    Load,
    Settings,
    Exit,
}

#[derive(Component, Reflect, Default, Clone, Inspectable, Serialize, Deserialize)]
#[reflect(Component)]
pub struct ImageTip {
    pub name: String,
    pub loaded: bool,
}

impl ImageTip {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), loaded: false }
    }
}
