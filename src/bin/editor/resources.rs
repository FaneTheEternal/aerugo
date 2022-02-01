use std::collections::HashMap;
use uuid::Uuid;
use aerugo::*;

pub struct AppData {
    pub(crate) file: String,
    pub(crate) aerugo: Aerugo,
}

pub struct AppState {
    pub(crate) current: Step,
}

pub trait AsOrigin {
    fn as_origin(&self) -> Steps;
}

#[derive(Default)]
pub struct TextStep {
    pub(crate) author: String,
    pub(crate) texts: String,
}

impl AsOrigin for TextStep {
    fn as_origin(&self) -> Steps {
        Steps::Text { author: self.author.clone(), texts: self.texts.clone() }
    }
}

#[derive(Default)]
pub struct JumpStep {
    pub(crate) condition: ConditionString,
    pub(crate) target: Uuid,
}

impl AsOrigin for JumpStep {
    fn as_origin(&self) -> Steps {
        Steps::Jump { condition: self.condition.clone().into(), target: self.target }
    }
}

#[derive(Default)]
pub struct PhraseStep {
    pub(crate) phrases: Vec<(String, String)>,
}

impl AsOrigin for PhraseStep {
    fn as_origin(&self) -> Steps {
        Steps::Phrase { phrases: self.phrases.clone() }
    }
}

#[derive(Default)]
pub struct ImageSelectStep {
    pub background: String,
    pub options: HashMap<String, (String, (isize, isize))>,
}

impl AsOrigin for ImageSelectStep {
    fn as_origin(&self) -> Steps {
        Steps::ImageSelect { background: self.background.clone(), options: self.options.clone() }
    }
}

#[derive(Default)]
pub struct SpriteNarratorStep {
    pub(crate) sprite: String,
}

impl AsOrigin for SpriteNarratorStep {
    fn as_origin(&self) -> Steps {
        Steps::SpriteNarrator {
            sprite: if self.sprite.is_empty() { None } else { Some(self.sprite.clone()) }
        }
    }
}

pub struct SpriteStep {
    pub(crate) name: String,
    pub(crate) sprite: String,
    pub(crate) animation: CommonAnimation,
}

impl AsOrigin for SpriteStep {
    fn as_origin(&self) -> Steps {
        Steps::Sprite {
            name: self.name.clone(),
            sprite: self.sprite.clone(),
            animation: self.animation.clone(),
        }
    }
}

impl Default for SpriteStep {
    fn default() -> Self {
        SpriteStep {
            name: Default::default(),
            sprite: Default::default(),
            animation: CommonAnimation::None,
        }
    }
}

pub struct BackgroundStep {
    pub(crate) command: BackgroundCommand,
}

impl AsOrigin for BackgroundStep {
    fn as_origin(&self) -> Steps {
        Steps::Background { command: self.command.clone() }
    }
}

impl Default for BackgroundStep {
    fn default() -> Self {
        BackgroundStep { command: BackgroundCommand::None }
    }
}

pub struct SceneStep {
    pub(crate) command: SceneCommand,
}

impl AsOrigin for SceneStep {
    fn as_origin(&self) -> Steps {
        Steps::Scene { command: self.command.clone() }
    }
}

impl Default for SceneStep {
    fn default() -> Self {
        SceneStep { command: SceneCommand::None }
    }
}

pub struct SaveEvent;

#[derive(Clone, Debug)]
pub struct ConditionString(pub(crate) String);

pub type ConditionStringInner = Option<Condition>;

impl Default for ConditionString {
    fn default() -> Self {
        ConditionStringInner::default().into()
    }
}

impl ConditionString {
    pub(crate) fn is_valid(&self) -> bool {
        serde_json::from_str::<'_, ConditionStringInner>(self.0.as_str()).is_ok()
    }
}

impl Into<ConditionStringInner> for ConditionString {
    fn into(self) -> ConditionStringInner {
        serde_json::from_str(self.0.as_str())
            .or_else::<serde_json::Error, _>(|_| { Ok(None) })
            .unwrap()
    }
}

impl From<ConditionStringInner> for ConditionString {
    fn from(c: ConditionStringInner) -> Self {
        ConditionString(serde_json::to_string(&c).unwrap())
    }
}

impl AppState {
    pub(crate) fn from_step(step: Step) -> AppState {
        AppState { current: step }
    }
}
