#![allow(dead_code)]

use std::io::Read;

use bevy::asset::{Asset, LoadState};
use bevy::ecs::{
    schedule::ShouldRun,
    system::SystemParam,
};
use bevy::prelude::*;

use aerugo::Aerugo;

use crate::startup::AssetCache;

pub const BTN_NORMAL: Color = Color::WHITE;
pub const BTN_HOVERED: Color = Color::GRAY;
pub const BTN_PRESSED: Color = Color::DARK_GRAY;

pub const TRANSPARENT: Color = Color::rgba(1.0, 1.0, 1.0, 0.0);
pub const GLASS_RED: Color = Color::rgba(1.0, 0.0, 0.0, 0.5);

pub const Z_SCENE: f32 = 15.0;
pub const Z_BACKGROUND: f32 = 5.0;
pub const Z_SPRITE: f32 = 10.0;
pub const Y_SPRITE: f32 = 0.0;

pub const SIZE_ALL: Size<Val> = Size {
    width: Val::Percent(100.0),
    height: Val::Percent(100.0),
};

pub const NARRATOR_DEFAULT: UiRect<Val> = UiRect {
    left: Val::Px(10.0),
    right: Val::Px(10.0),
    top: Val::Px(10.0),
    bottom: Val::Undefined,
};
pub const NARRATOR_SHIFT: UiRect<Val> = UiRect {
    left: Val::Px(10.0 + NARRATOR_SIDE),
    right: Val::Px(10.0),
    top: Val::Px(10.0),
    bottom: Val::Undefined,
};
pub const FLOW_DEFAULT: UiRect<Val> = UiRect {
    left: Val::Px(10.0),
    right: Val::Px(10.0),
    top: Val::Px(40.0),
    bottom: Val::Px(10.0),
};
pub const FLOW_SHIFT: UiRect<Val> = UiRect {
    left: Val::Px(10.0 + NARRATOR_SIDE),
    right: Val::Px(10.0),
    top: Val::Px(40.0),
    bottom: Val::Px(10.0),
};
const NARRATOR_SIDE: f32 = 200.0;
pub const NARRATOR_FRAME: Size<Val> = Size {
    width: Val::Px(NARRATOR_SIDE),
    height: Val::Px(NARRATOR_SIDE),
};
pub const FLOW_MAX_DEFAULT: Size<Val> = Size {
    width: Val::Px(900.0),
    height: Val::Undefined,
};
pub const FLOW_MAX_SHIFT: Size<Val> = Size {
    width: Val::Px(900.0 - NARRATOR_SIDE),
    height: Val::Undefined,
};

pub fn grow_z_index<'closure>(
    deep: u8,
    builder: &mut ChildBuilder,
    tree_style: Style,
    closure: impl FnOnce(&mut ChildBuilder) + 'closure,
)
{
    builder
        .spawn_bundle(NodeBundle {
            style: tree_style.clone(),
            color: TRANSPARENT.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            if deep == 0 {
                parent
                    .spawn_bundle(NodeBundle {
                        style: tree_style.clone(),
                        color: TRANSPARENT.into(),
                        ..Default::default()
                    })
                    .with_children(closure);
            } else {
                grow_z_index(deep - 1, parent, tree_style, closure);
            }
        });
}

pub fn load_aerugo() -> Aerugo {
    const SCENARIO_PATH: &str = "scenario.ron";
    let mut file = std::fs::File::options()
        .read(true).write(true).create(true)
        .open(SCENARIO_PATH)
        .unwrap();
    let mut aerugo = String::new();
    file.read_to_string(&mut aerugo).unwrap();
    ron::from_str(&aerugo).unwrap()
}

#[allow(dead_code)]
pub fn run_once_criteria() -> impl FnMut() -> ShouldRun {
    let mut ran = false;
    move || {
        if ran {
            ShouldRun::No
        } else {
            ran = true;
            ShouldRun::Yes
        }
    }
}


pub const BTN_FONT: &str = r"fonts/FiraSans-Bold.ttf";
pub const MAIN_BACK: &str = r"hud/mm_back.png";
pub const MAIN_BTN_BACK: &str = r"hud/mm_btn_back.png";
pub const MAIN_BTN_HOVER: &str = r"hud/mm_btn_hover.png";

pub const BTN1: &str = r"hud/mm_btn1.png";
pub const BTN2: &str = r"hud/mm_btn2.png";
pub const BTN3: &str = r"hud/mm_btn3.png";
pub const BTN4: &str = r"hud/mm_btn4.png";
pub const BTN5: &str = r"hud/mm_btn5.png";
pub const BTN6: &str = r"hud/mm_btn6.png";

#[derive(SystemParam)]
pub struct CachedAssetServer<'w, 's> {
    asset_server: Res<'w, AssetServer>,
    cache: ResMut<'w, AssetCache>,
    _s: Query<'w, 's, ()>,
}

impl<'w, 's> CachedAssetServer<'w, 's> {
    pub fn load_untyped(&mut self, path: &str) -> HandleUntyped {
        if let Some(handle) = self.cache.assets.get(path) {
            handle.clone()
        } else {
            let handle = self.asset_server.load_untyped(path);
            self.cache.assets.insert(path.replace(r"/", r"\"), handle.clone());
            self.cache.assets.insert(path.replace(r"\", r"/"), handle.clone());
            handle
        }
    }

    pub fn load<T: Asset>(&mut self, path: &str) -> Handle<T> {
        self.load_untyped(path).typed()
    }

    pub fn all_loaded(&self) -> bool {
        let id_iter = self.cache.assets.values().map(|h| h.id);
        match self.asset_server.get_group_load_state(id_iter) {
            LoadState::NotLoaded => { false }
            LoadState::Loading => { false }
            LoadState::Loaded => { true }
            LoadState::Failed => { true }  // maybe false
            LoadState::Unloaded => { unreachable!() }
        }
    }
}

pub const FONT_DIALOG: &str = "fonts/CormorantGaramond-SemiBold.ttf";
pub const FONT_NAME: &str = "fonts/CormorantGaramond-SemiBold.ttf";
pub const FONT_FLOW: &str = "fonts/CormorantGaramond-SemiBold.ttf";
pub const FONT_MAIN_MENU: &str = "fonts/Bitter-Bold.ttf";
pub const FONT_DEFAULT: &str = "fonts/Bitter-Medium.ttf";
