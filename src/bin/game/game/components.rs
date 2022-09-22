use bevy::prelude::*;

// region text flow
#[derive(Debug, Clone, Eq, PartialEq, Hash, Component)]
pub struct TextFlowBase;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Component)]
pub struct TextFlowMark;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Component)]
pub struct NarratorFlowMark;

#[derive(Debug, Clone, Component)]
pub struct AnimateText {
    pub text: String,
    pub timer: Timer,
    pub style: TextStyle,
    pub chars: usize,
}
// endregion

// region phrase
#[derive(Debug, Clone, Eq, PartialEq, Hash, Component)]
pub struct PhraseValue(pub String);
// endregion

// region narrator
#[derive(Debug, Clone, Eq, PartialEq, Hash, Component)]
pub struct NarratorMark;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Component)]
pub struct NarratorPlaceholderMark;
// endregion

// region background
#[derive(Debug, Clone, Eq, PartialEq, Hash, Component)]
pub struct BackgroundMark;
// endregion

// region scene
#[derive(Debug, Clone, Eq, PartialEq, Hash, Component)]
pub struct SceneMark;
// endregion

// region sprite
#[derive(Debug, Clone, Component)]
pub struct AnimateFadeSprite {
    pub timer: Timer,
    pub fade_in: bool,
    pub name: String,
}

#[derive(Debug, Clone, Component)]
pub struct AnimateMoveSprite {
    pub timer: Timer,
    pub start_pos: f32,
    pub end_pos: f32,
    pub name: String,
    pub move_out: bool,
}
// endregion


#[derive(Debug, Clone, Component)]
pub struct AnimateScene {
    pub timer: Timer,
    pub is_loop: bool,
    pub is_paused: bool,
}
