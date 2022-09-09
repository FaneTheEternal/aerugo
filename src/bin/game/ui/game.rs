use bevy::prelude::*;
use crate::utils::{FLOW_DEFAULT, FLOW_SHIFT, NARRATOR_DEFAULT, NARRATOR_SHIFT};

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
        asset_server: &AssetServer,
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
            self._fix_narrator_and_flow(style_query);
        } else {
            warn!("Unknown narrator name: {:?}", name);
        }
    }

    fn _fix_narrator_and_flow(
        &self,
        style_query: &mut Query<&mut Style>,
    )
    {
        let mut not_hide = false;
        for (name, narrator) in &self.narrator_sprites {
            if name.eq("second") { continue; }
            let style = style_query.get(narrator.root).unwrap();
            not_hide |= style.display == Display::Flex;
        }
        if not_hide {
            style_query.get_mut(self.narrator_base).unwrap()
                .margin = NARRATOR_SHIFT;
            style_query.get_mut(self.text_base).unwrap()
                .padding = FLOW_SHIFT;
        } else {
            style_query.get_mut(self.narrator_base).unwrap()
                .margin = NARRATOR_DEFAULT;
            style_query.get_mut(self.text_base).unwrap()
                .padding = FLOW_DEFAULT;
        }
    }

    pub fn clean_narrators(
        &self,
        style_query: &mut Query<&mut Style>,
        image_query: &mut Query<&mut UiImage>,
    )
    {
        for (_, narrator) in &self.narrator_sprites {
            style_query.get_mut(narrator.root.clone()).unwrap()
                .display = Display::None;
            image_query.get_mut(narrator.img.clone()).unwrap()
                .0 = default();
        }
        self._fix_narrator_and_flow(style_query);
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

    pub fn force_hide(&mut self, query: &mut Query<&mut Style>) {
        self.is_visible = false;
        self._hide(query);
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
    pub(crate) menu: Entity,
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
    fn _show_menu(&self, query: &mut Query<&mut Style>) {
        query.get_mut(self.menu).unwrap().display = Display::Flex;
    }
    fn _hide_menu(&self, query: &mut Query<&mut Style>) {
        query.get_mut(self.menu).unwrap().display = Display::None;
    }

    pub fn show(&self, mut query: Query<&mut Style>, mut query_2d: Query<&mut Visibility>) {
        self._show_game(&mut query, &mut query_2d);
    }

    pub fn hide(&self, mut query: Query<&mut Style>, mut query_2d: Query<&mut Visibility>) {
        self._hide_game(&mut query, &mut query_2d);
    }

    pub fn show_menu(&self, mut query: Query<&mut Style>) {
        self._show_menu(&mut query);
    }

    pub fn hide_menu(&self, mut query: Query<&mut Style>) {
        self._hide_menu(&mut query);
    }

    #[allow(dead_code)]
    pub fn show_all(&self, mut query: Query<&mut Style>, mut query_2d: Query<&mut Visibility>) {
        self._show_game(&mut query, &mut query_2d);
        self._show_menu(&mut query);
    }

    pub fn hide_all(&self, mut query: Query<&mut Style>, mut query_2d: Query<&mut Visibility>) {
        self._hide_game(&mut query, &mut query_2d);
        self._hide_menu(&mut query);
    }

    pub fn smart_show(
        &self,
        mut query: Query<&mut Style>,
        mut query_2d: Query<&mut Visibility>,
        game_state: &GameState)
    {
        self._show_game(&mut query, &mut query_2d);
        if game_state.eq(&GameState::Paused) {
            self._show_menu(&mut query);
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
