use bevy::prelude::*;


// region text flow
#[derive(Debug, Clone, Eq, PartialEq, Hash, Component)]
pub struct TextFlowBase;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Component)]
pub struct TextFlowMark;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Component)]
pub struct NarratorFlowMark;
// endregion

// region narrator
#[derive(Debug, Clone, Eq, PartialEq, Hash, Component)]
pub struct NarratorMark;
// endregion

// region background
#[derive(Debug, Clone, Eq, PartialEq, Hash, Component)]
pub struct BackgroundMark;
// endregion
