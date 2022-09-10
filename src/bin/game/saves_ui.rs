use bevy::prelude::*;
use crate::saves::{LoadMark, SaveMark, Saves};
use crate::utils::{SIZE_ALL, TRANSPARENT};

#[derive(Component)]
pub struct SaveItemsParentMark;

#[derive(Component)]
pub struct LoadItemsParentMark;
