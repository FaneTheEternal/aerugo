use bevy::prelude::*;

use crate::utils::*;

use super::*;

#[derive(Debug, Clone)]
pub struct NarratorUI {
    pub root: Entity,
    pub img: Entity,
}

#[allow(dead_code)]
pub struct TextUI {
    pub(crate) root: Entity,
    pub(crate) is_visible: bool,
    pub(crate) flow: Entity,
    pub(crate) narrator: Entity,
    pub(crate) text: Entity,
    pub(crate) narrator_sprites: HashMap<String, NarratorUI>,

    pub(crate) narrator_base: Entity,
    pub(crate) text_base: Entity,
}

impl TextUI {
    pub fn set_narrator(
        &self,
        style_query: &mut Query<&mut Style>,
        image_query: &mut Query<&mut UiImage>,
        name: &str,
        sprite: Option<String>,
        asset_server: &mut CachedAssetServer,
        window: &Window,
    )
    {
        if let Some(narrator) = self.narrator_sprites.get(name) {
            match sprite {
                None => {
                    style_query.get_mut(narrator.root.clone()).unwrap()
                        .display = Display::None;
                    image_query.get_mut(narrator.img.clone()).unwrap()
                        .0 = default();
                }
                Some(sprite) => {
                    style_query.get_mut(narrator.root.clone()).unwrap()
                        .display = Display::Flex;
                    image_query.get_mut(narrator.img.clone()).unwrap()
                        .0 = asset_server.load(&sprite).into();
                }
            };
            self._fix_narrator_and_flow(style_query, window);
        } else {
            warn!("Unknown narrator name: {:?}", name);
        }
    }

    fn narrator_visible(&self, style_query: &mut Query<&mut Style>) -> bool {
        let mut not_hide = false;
        for (name, narrator) in &self.narrator_sprites {
            if name.eq("second") { continue; }
            let style = style_query.get(narrator.root).unwrap();
            not_hide |= style.display == Display::Flex;
        }
        not_hide
    }

    fn _fix_narrator_and_flow(
        &self,
        style_query: &mut Query<&mut Style>,
        window: &Window,
    )
    {
        if self.narrator_visible(style_query) {
            style_query.get_mut(self.narrator_base).unwrap()
                .margin = get_narrator_shift(window.height());
            style_query.get_mut(self.text_base).unwrap()
                .padding = get_flow_shift(window.height());
        } else {
            style_query.get_mut(self.narrator_base).unwrap()
                .margin = NARRATOR_DEFAULT;
            style_query.get_mut(self.text_base).unwrap()
                .padding = FLOW_DEFAULT;
        };
        self.resize_relative(style_query, window.width(), window.height());
    }

    pub fn clean_narrators(
        &self,
        style_query: &mut Query<&mut Style>,
        image_query: &mut Query<&mut UiImage>,
        window: &Window,
    )
    {
        for (_, narrator) in &self.narrator_sprites {
            style_query.get_mut(narrator.root.clone()).unwrap()
                .display = Display::None;
            image_query.get_mut(narrator.img.clone()).unwrap()
                .0 = default();
        }
        self._fix_narrator_and_flow(style_query, window);
    }

    fn _show(&self, query: &mut Query<&mut Style>) {
        query.get_mut(self.root).unwrap().display = Display::Flex;
    }

    pub fn show(&self, query: &mut Query<&mut Style>) {
        if self.is_visible { self._show(query) }
    }

    pub fn force_show(&mut self, query: &mut Query<&mut Style>) {
        self.is_visible = true;
        self._show(query);
    }

    pub fn _hide(&self, query: &mut Query<&mut Style>) {
        query.get_mut(self.root).unwrap().display = Display::None;
    }

    #[allow(dead_code)]
    pub fn hide(&self, query: &mut Query<&mut Style>) {
        if self.is_visible { self._hide(query) }
    }

    #[allow(dead_code)]
    pub fn force_hide(&mut self, query: &mut Query<&mut Style>) {
        self.is_visible = false;
        self._hide(query);
    }

    pub fn resize_relative(&self, style_query: &mut Query<&mut Style>, width: f32, height: f32)
    {
        let flow_width = Self::get_flow_width(width, self.narrator_visible(style_query));
        style_query.get_mut(self.text).unwrap()
            .max_size.width = Val::Px(flow_width);
        let narrator_side = get_narrator_side(height);
        for narrator in self.narrator_sprites.values() {
            style_query.get_mut(narrator.root).unwrap()
                .size = Size::new(Val::Px(narrator_side), Val::Px(narrator_side));
        }
    }

    fn get_flow_width(width: f32, expanded: bool) -> f32 {
        let width = width * 0.75 - 30.0;
        let shift = if expanded { get_narrator_side(width) + 10.0 } else { 0.0 };
        width - shift
    }
}

pub struct PhraseUI {
    pub(crate) root: Entity,
    pub(crate) is_visible: bool,
}

impl PhraseUI {
    fn _show(&self, query: &mut Query<&mut Style>) {
        query.get_mut(self.root).unwrap().display = Display::Flex;
    }

    pub fn show(&self, query: &mut Query<&mut Style>) {
        if self.is_visible { self._show(query) }
    }

    pub fn force_show(&mut self, query: &mut Query<&mut Style>) {
        self.is_visible = true;
        self._show(query);
    }

    pub fn _hide(&self, query: &mut Query<&mut Style>) {
        query.get_mut(self.root).unwrap().display = Display::None;
    }

    #[allow(dead_code)]
    pub fn hide(&self, query: &mut Query<&mut Style>) {
        if self.is_visible { self._hide(query) }
    }

    pub fn force_hide(&mut self, query: &mut Query<&mut Style>) {
        self.is_visible = false;
        self._hide(query);
    }
}

pub struct GameUI {
    pub(crate) ui_root: Entity,
    pub(crate) background: Entity,
    pub(crate) background_visible: bool,
    pub(crate) scene: Entity,
    pub(crate) scene_visible: bool,
    pub(crate) sprites: HashMap<String, Entity>,

    pub(crate) text: TextUI,
    pub(crate) phrase: PhraseUI,
}

impl GameUI {
    fn _show_game(&self, query: &mut Query<&mut Style>, query_2d: &mut Query<&mut Visibility>) {
        self.text.show(query);
        self.phrase.show(query);
        query.get_mut(self.ui_root).unwrap().display = Display::Flex;
        if self.background_visible {
            query_2d.get_mut(self.background).unwrap().is_visible = true;
        }
        if self.scene_visible {
            query_2d.get_mut(self.scene).unwrap().is_visible = true;
        }
        self.sprites.values().for_each(|e| {
            query_2d.get_mut(*e).unwrap().is_visible = true;
        });
    }
    fn _hide_game(&self, query: &mut Query<&mut Style>, query_2d: &mut Query<&mut Visibility>) {
        query.get_mut(self.ui_root).unwrap().display = Display::None;
        query_2d.get_mut(self.background).unwrap().is_visible = false;
        query_2d.get_mut(self.scene).unwrap().is_visible = false;
        self.sprites.values().for_each(|e| {
            query_2d.get_mut(*e).unwrap().is_visible = false;
        });
    }

    pub fn show(&self, mut query: Query<&mut Style>, mut query_2d: Query<&mut Visibility>) {
        self._show_game(&mut query, &mut query_2d);
    }

    pub fn hide(&self, mut query: Query<&mut Style>, mut query_2d: Query<&mut Visibility>) {
        self._hide_game(&mut query, &mut query_2d);
    }

    #[allow(dead_code)]
    pub fn show_all(&self, mut query: Query<&mut Style>, mut query_2d: Query<&mut Visibility>) {
        self._show_game(&mut query, &mut query_2d);
    }

    pub fn hide_all(&self, mut query: Query<&mut Style>, mut query_2d: Query<&mut Visibility>) {
        self._hide_game(&mut query, &mut query_2d);
    }

    pub fn smart_show(
        &self,
        mut query: Query<&mut Style>,
        mut query_2d: Query<&mut Visibility>,
        game_state: &GameState,
    )
    {
        match game_state {
            GameState::None | GameState::Paused => {}
            GameState::Init | GameState::Active => {
                self._show_game(&mut query, &mut query_2d);
            }
        }
    }

    pub fn resize_relative(
        &self,
        sprite_query: &mut Query<&mut Sprite>,
        atlas_query: &mut Query<&mut TextureAtlasSprite>,
        width: f32,
        height: f32,
    )
    {
        sprite_query.get_mut(self.background).unwrap()
            .custom_size = Some(Vec2::new(width, height));
        if let Ok(mut sprite) = sprite_query.get_mut(self.scene) {
            sprite.custom_size = Some(Vec2::new(width, height));
        }
        if let Ok(mut atlas_sprite) = atlas_query.get_mut(self.scene) {
            atlas_sprite.custom_size = Some(Vec2::new(width, height));
        }
        for sprite in self.sprites.values() {
            sprite_query.get_mut(*sprite).unwrap()
                .custom_size = Some(Vec2::new(width, height));
        }
    }
}

pub fn game_show(
    game_ui: Res<GameUI>,
    query: Query<&mut Style>,
    query_2d: Query<&mut Visibility>,
    game_state: Res<State<GameState>>,
)
{
    game_ui.smart_show(query, query_2d, game_state.current());
}

pub fn game_hide(
    game_ui: Res<GameUI>,
    query: Query<&mut Style>,
    query_2d: Query<&mut Visibility>,
)
{
    game_ui.hide_all(query, query_2d);
}
