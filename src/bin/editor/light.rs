use strum_macros::EnumIter;

use aerugo::*;


#[derive(EnumIter, Debug, Clone, Default, Eq, PartialEq)]
pub enum LightInner {
    Text,
    Jump,
    Phrase,
    ImageSelect,
    SpriteNarrator,
    Sprite,
    Background,
    Scene,
    #[default]
    None,
}

impl From<Steps> for LightInner {
    fn from(steps: Steps) -> Self {
        match steps {
            Steps::Text { .. } => { LightInner::Text }
            Steps::Jump { .. } => { LightInner::Jump }
            Steps::Phrase { .. } => { LightInner::Phrase }
            Steps::ImageSelect { .. } => { LightInner::ImageSelect }
            Steps::SpriteNarrator { .. } => { LightInner::SpriteNarrator }
            Steps::Sprite(_) => { LightInner::Sprite }
            Steps::Background(_) => { LightInner::Background }
            Steps::Scene(_) => { LightInner::Scene }
            Steps::None => { LightInner::None }
        }
    }
}

impl Into<Steps> for LightInner {
    fn into(self) -> Steps {
        match self {
            LightInner::Text => {
                Steps::Text { author: "".to_string(), texts: "".to_string() }
            }
            LightInner::Jump => {
                Steps::Jump { condition: None, target: Default::default() }
            }
            LightInner::Phrase => {
                Steps::Phrase { phrases: vec![] }
            }
            LightInner::ImageSelect => {
                Steps::ImageSelect { background: "".to_string(), options: Default::default() }
            }
            LightInner::SpriteNarrator => {
                Steps::SpriteNarrator(NarratorCommand::None)
            }
            LightInner::Sprite => {
                Steps::Sprite(SpriteCommand::None)
            }
            LightInner::Background => {
                Steps::Background(BackgroundCommand::None)
            }
            LightInner::Scene => {
                Steps::Scene(SceneCommand::None)
            }
            LightInner::None => {
                Steps::None
            }
        }
    }
}


#[derive(EnumIter, Debug, Clone, Default, Eq, PartialEq)]
pub enum NarratorLight {
    Set,
    Remove,
    Clean,
    #[default]
    None,
}

impl From<NarratorCommand> for NarratorLight {
    fn from(cmd: NarratorCommand) -> Self {
        match cmd {
            NarratorCommand::Set { .. } => { NarratorLight::Set }
            NarratorCommand::Remove { .. } => { NarratorLight::Remove }
            NarratorCommand::Clean => { NarratorLight::Clean }
            NarratorCommand::None => { NarratorLight::None }
        }
    }
}

impl Into<NarratorCommand> for NarratorLight {
    fn into(self) -> NarratorCommand {
        match self {
            NarratorLight::Set => {
                NarratorCommand::Set { name: "".to_string(), sprite: "".to_string() }
            }
            NarratorLight::Remove => {
                NarratorCommand::Remove { name: "".to_string() }
            }
            NarratorLight::Clean => {
                NarratorCommand::Clean
            }
            NarratorLight::None => {
                NarratorCommand::None
            }
        }
    }
}


#[derive(EnumIter, Debug, Clone, Default, Eq, PartialEq)]
pub enum SpriteLight {
    #[default]
    None,
    Set,
    Remove,
    FadeIn,
    FadeOut,
    LeftIn,
    LeftOut,
    RightIn,
    RightOut,
    Move,
}

impl From<SpriteCommand> for SpriteLight {
    fn from(cmd: SpriteCommand) -> Self {
        match cmd {
            SpriteCommand::None => { SpriteLight::None }
            SpriteCommand::Set { .. } => { SpriteLight::Set }
            SpriteCommand::Remove { .. } => { SpriteLight::Remove }
            SpriteCommand::FadeIn { .. } => { SpriteLight::FadeIn }
            SpriteCommand::FadeOut { .. } => { SpriteLight::FadeOut }
            SpriteCommand::LeftIn { .. } => { SpriteLight::LeftIn }
            SpriteCommand::LeftOut { .. } => { SpriteLight::LeftOut }
            SpriteCommand::RightIn { .. } => { SpriteLight::RightIn }
            SpriteCommand::RightOut { .. } => { SpriteLight::RightOut }
            SpriteCommand::Move { .. } => { SpriteLight::Move }
        }
    }
}

impl Into<SpriteCommand> for SpriteLight {
    fn into(self) -> SpriteCommand {
        match self {
            SpriteLight::None => {
                SpriteCommand::None
            }
            SpriteLight::Set => {
                SpriteCommand::Set { name: "".to_string(), sprite: "".to_string(), position: 0.0 }
            }
            SpriteLight::Remove => {
                SpriteCommand::Remove { name: "".to_string() }
            }
            SpriteLight::FadeIn => {
                SpriteCommand::FadeIn { sprite: "".to_string(), name: "".to_string(), position: 0.0 }
            }
            SpriteLight::FadeOut => {
                SpriteCommand::FadeOut { name: "".to_string() }
            }
            SpriteLight::LeftIn => {
                SpriteCommand::LeftIn { sprite: "".to_string(), name: "".to_string(), position: 0.0 }
            }
            SpriteLight::LeftOut => {
                SpriteCommand::LeftOut { name: "".to_string() }
            }
            SpriteLight::RightIn => {
                SpriteCommand::RightIn { sprite: "".to_string(), name: "".to_string(), position: 0.0 }
            }
            SpriteLight::RightOut => {
                SpriteCommand::RightOut { name: "".to_string() }
            }
            SpriteLight::Move => {
                SpriteCommand::Move { name: "".to_string(), position: 0.0 }
            }
        }
    }
}


#[derive(EnumIter, Debug, Clone, Default, Eq, PartialEq)]
pub enum BackgroundLight {
    #[default]
    None,
    Change,
    Shake,
}

impl From<BackgroundCommand> for BackgroundLight {
    fn from(cmd: BackgroundCommand) -> Self {
        match cmd {
            BackgroundCommand::Change { .. } => { BackgroundLight::Change }
            BackgroundCommand::Shake => { BackgroundLight::Shake }
            BackgroundCommand::None => { BackgroundLight::None }
        }
    }
}

impl Into<BackgroundCommand> for BackgroundLight {
    fn into(self) -> BackgroundCommand {
        match self {
            BackgroundLight::None => { BackgroundCommand::None }
            BackgroundLight::Change => {
                BackgroundCommand::Change { new: "".to_string(), animation: None }
            }
            BackgroundLight::Shake => { BackgroundCommand::Shake }
        }
    }
}


#[derive(EnumIter, Debug, Clone, Default, Eq, PartialEq)]
pub enum SceneLight {
    #[default]
    None,
    Set,
    Remove,
    Play,
    Pause,
    Resume,
    Stop,
}

impl From<SceneCommand> for SceneLight {
    fn from(cmd: SceneCommand) -> Self {
        match cmd {
            SceneCommand::Set { .. } => { SceneLight::Set }
            SceneCommand::Remove => { SceneLight::Remove }
            SceneCommand::Play { .. } => { SceneLight::Play }
            SceneCommand::Pause => { SceneLight::Pause }
            SceneCommand::Resume => { SceneLight::Resume }
            SceneCommand::Stop => { SceneLight::Stop }
            SceneCommand::None => { SceneLight::None }
        }
    }
}

impl Into<SceneCommand> for SceneLight {
    fn into(self) -> SceneCommand {
        match self {
            SceneLight::None => { SceneCommand::None }
            SceneLight::Set => { SceneCommand::Set { name: "".to_string() } }
            SceneLight::Remove => { SceneCommand::Remove }
            SceneLight::Play => {
                SceneCommand::Play {
                    name: "".to_string(),
                    is_loop: false,
                    tile: (0, 0),
                    columns: 0,
                    rows: 0,
                }
            }
            SceneLight::Pause => { SceneCommand::Pause }
            SceneLight::Resume => { SceneCommand::Resume }
            SceneLight::Stop => { SceneCommand::Stop }
        }
    }
}
