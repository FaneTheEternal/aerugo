use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Hash, Serialize, Deserialize)]
pub enum ImanityLangs {
    #[default]
    Ru,
    En,
    // TODO: others
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct AerugoImanity {
    pub lang: ImanityLangs,
    pub localized: Vec<Step>,
}

pub struct Internationale {
    pub defs: Vec<AerugoImanity>,
}

impl Internationale {
    pub fn load() -> Internationale {
        let base = std::env::current_dir().unwrap()
            .join("assets")
            .join("lang");
        let mut defs = Vec::new();

        std::fs::create_dir_all(&base).unwrap();
        for entry in std::fs::read_dir(&base).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            let metadata = std::fs::metadata(&path).unwrap();
            if metadata.is_file() {
                let name = path.strip_prefix(&base).unwrap();
                let name = name.to_string_lossy();
                if let Some(_) = name.strip_suffix(".imanity") {
                    let imanity = std::fs::read_to_string(path).unwrap();
                    let imanity: AerugoImanity = ron::from_str(&imanity).unwrap();
                    println!("Load AerugoImanity::{:?}", imanity.lang);
                    defs.push(imanity);
                }
            }
        }

        Internationale {
            defs
        }
    }

    pub fn adapt(&self, lang: ImanityLangs, aerugo: &mut Aerugo) {
        let localized = self.defs.iter().find(|def| def.lang == lang);
        if let Some(localized) = localized {
            for step in aerugo.steps.iter_mut() {
                if let Some(local) = localized.localized.iter().find(|s| s.id == step.id) {
                    step.inner = local.inner.clone();
                } else if let Steps::Text {..} = step.inner {
                    println!("Missing localized step({}): {:?}", step.id, step.inner);
                } else if let Steps::Phrase {..} = step.inner {
                    println!("Missing localized step({}): {:?}", step.id, step.inner);
                }
            }
        } else {
            println!("Unknown localization, generated defaults...")
        }
    }
}