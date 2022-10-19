use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use crate::translator::{Lang};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub lang: Lang,
    #[serde(default)]
    pub resolution: Resolution,
    #[serde(default = "Defs::narrator_size")]
    pub narrator_size: f32,
    #[serde(default = "Defs::flow_speed")]
    pub flow_speed: f32,
    #[serde(default = "Defs::flow_size")]
    pub flow_size: f32,
}

struct Defs;

impl Defs {
    fn narrator_size() -> f32 { 30.0 }
    fn flow_speed() -> f32 { 0.1 }
    fn flow_size() -> f32 { 31.0 }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            lang: default(),
            resolution: default(),
            narrator_size: Defs::narrator_size(),
            flow_speed: Defs::flow_speed(),
            flow_size: Defs::flow_size(),
        }
    }
}

impl Settings {
    pub fn load() -> Settings {
        let path = std::env::current_dir().unwrap()
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

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum Resolution {
    #[default]
    HD,
    FHD,
    QHD,
}

impl Resolution {
    pub fn get(&self) -> (f32, f32) {
        match self {
            Resolution::HD => { (1280.0, 720.0) }
            Resolution::FHD => { (1920.0, 1080.0) }
            Resolution::QHD => { (2560.0, 1440.0) }
        }
    }

    pub fn verbose(&self) -> String {
        let (w, h) = self.get();
        format!("{:?} ({}:{})", self, w, h)
    }
}
