use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io::Write;
use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use aerugo::Aerugo;
use aerugo::international::{ImanityLangs, Internationale};
use crate::settings::Settings;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LangDict(HashMap<String, String>);

impl LangDict {
    pub fn dump(&self, lang: &Lang) {
        let data = ron::ser::to_string_pretty(self, default()).unwrap();
        let path = std::env::current_dir().unwrap()
            .join("assets")
            .join("lang")
            .join(format!("{:?}.hud", lang).to_lowercase());
        std::fs::File::options()
            .write(true).create(true).truncate(true)
            .open(path).unwrap()
            .write_all(data.as_bytes())
            .unwrap()
    }
}

#[derive(Debug, Clone, Default, Resource)]
pub struct Translator {
    defs: HashMap<Lang, LangDict>,
}

impl Translator {
    const EXT: &'static str = ".hud";

    pub fn load() -> Translator {
        // TODO: as asset
        let base = std::env::current_dir().unwrap()
            .join("assets")
            .join("lang");
        let mut defs = HashMap::<Lang, LangDict>::default();
        defs.insert(Lang::Ru, default());
        defs.insert(Lang::En, default());
        std::fs::create_dir_all(&base).unwrap();
        for entry in std::fs::read_dir(&base).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            let metadata = std::fs::metadata(&path).unwrap();
            if metadata.is_file() {
                let name = path.strip_prefix(&base).unwrap();
                let name = name.to_string_lossy();
                if let Some(name) = name.strip_suffix(Self::EXT) {
                    match name.to_lowercase().as_str() {
                        "ru" => {
                            let data = std::fs::read_to_string(path).unwrap();
                            defs.insert(Lang::Ru, ron::from_str(&data).unwrap());
                        }
                        "en" => {
                            let data = std::fs::read_to_string(path).unwrap();
                            defs.insert(Lang::En, ron::from_str(&data).unwrap());
                        }
                        _ => { warn!("Unimplemented lang {:?}", name) }
                    }
                }
            }
        }
        Self { defs }
    }

    pub fn get(&mut self, lang: &Lang, origin: &str) -> String {
        if let Some(dict) = self.defs.get_mut(&lang) {
            return match dict.0.entry(origin.into()) {
                Entry::Occupied(translated) => {
                    translated.get().clone()
                }
                Entry::Vacant(v) => {
                    v.insert(origin.into());
                    dict.dump(lang);
                    origin.into()
                }
            };
        }
        origin.into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum Lang {
    // TODO: All languages
    Ru,
    #[default]
    En,
}

impl From<Lang> for ImanityLangs {
    fn from(l: Lang) -> Self {
        match l {
            Lang::Ru => { ImanityLangs::Ru }
            Lang::En => { ImanityLangs::En }
        }
    }
}

#[derive(Debug, Clone, Component)]
pub struct TranslatableText;

#[derive(Debug, Clone, Component)]
pub struct TranslatedText(String);  // Origin

pub struct NewLang(pub Lang);


pub fn setup_text(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Text), With<TranslatableText>>,
    settings: Res<Settings>,
    mut translator: ResMut<Translator>,
    mut aerugo: ResMut<Aerugo>,
    internationale: Res<Internationale>,
)
{
    internationale.adapt(settings.lang.clone().into(), aerugo.as_mut());
    for (entity, text) in query.iter_mut() {
        let mut text: Mut<Text> = text;
        let origin = text.sections.get_mut(0).unwrap();
        let mut translated = translator.get(&settings.lang, &origin.value);

        std::mem::swap(&mut origin.value, &mut translated);  // !!!
        commands.entity(entity)
            .remove::<TranslatableText>()
            .insert(TranslatedText(translated));
    }
}

pub fn translate_text(
    mut query: Query<(&TranslatedText, &mut Text)>,
    mut settings: ResMut<Settings>,
    mut translator: ResMut<Translator>,
    mut events: EventReader<NewLang>,
    mut aerugo: ResMut<Aerugo>,
    internationale: Res<Internationale>,
)
{
    for event in events.iter() {
        settings.lang = event.0.clone();
        internationale.adapt(settings.lang.clone().into(), aerugo.as_mut());

        for (origin, text) in query.iter_mut() {
            let mut text: Mut<Text> = text;
            let origin: &TranslatedText = origin;
            text.sections.get_mut(0).unwrap()
                .value = translator.get(&settings.lang, &origin.0);
        }
    }
}
