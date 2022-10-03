use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::utils::*;
use serde::{Serialize, Deserialize};
use crate::translator::{Lang, Translator};


#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub lang: Lang,
    #[serde(default)]
    pub flow_speed: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            lang: Lang::En,
            flow_speed: 0.1,
        }
    }
}

impl Settings {
    pub fn load() -> Settings {
        let path = std::env::current_dir().unwrap()
            .join("assets")
            .join("settings.ron");
        let settings = if let Ok(data) = std::fs::read_to_string(&path) {
            if let Ok(settings) = ron::from_str(&data) {
                settings
            } else { default() }
        } else { default() };
        std::fs::write(
            &path,
            ron::ser::to_string_pretty(
                &settings,
                default(),
            ).unwrap(),
        ).unwrap();
        settings
    }

    pub fn dump(&self) {
        let path = std::env::current_dir().unwrap()
            .join("assets")
            .join("settings.ron");
        std::fs::write(
            &path,
            ron::ser::to_string_pretty(
                self,
                default(),
            ).unwrap(),
        ).unwrap();
    }
}