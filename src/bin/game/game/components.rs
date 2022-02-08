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
// endregion

// region background
#[derive(Debug, Clone, Eq, PartialEq, Hash, Component)]
pub struct BackgroundMark;
// endregion
