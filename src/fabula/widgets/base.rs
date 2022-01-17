#![allow(dead_code)]

use crate::widgets::prelude::*;

use super::read::*;

pub const FABULA_GAME: _SFClosure = |context, key| {
    let context = context;
    let root_key = key;

    READ_SCREEN(context, root_key)
};
