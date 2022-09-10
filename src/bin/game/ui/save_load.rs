use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::utils::tracing::{Level, span};

use aerugo::bevy_glue::SavePageButton;
use crate::saves::{LoadMark, SaveMark, Saves};
use crate::ui::UiState;

use crate::utils::TRANSPARENT;

pub struct SaveLoadUI {
    pub root: Entity,
    pub page_header: Entity,
    pub current: String,
    pub save_frames: Vec<SaveFrameUI>,
}

impl SaveLoadUI {
    pub fn show(&self, style_query: &mut Query<&mut Style>) {
        style_query.get_mut(self.root).unwrap().display = Display::Flex;
    }

    pub fn hide(&self, style_query: &mut Query<&mut Style>) {
        style_query.get_mut(self.root).unwrap().display = Display::None;
    }

    pub fn select_page(
        &mut self,
        page: &str,
        text_query: &mut Query<&mut Text>,
        img_query: &mut Query<&mut UiImage>,
        color_query: &mut Query<&mut UiColor>,
        saves: &Saves,
        asset_server: &AssetServer,
    )
    {
        self.current = page.into();
        let mut text = text_query.get_mut(self.page_header).unwrap();
        let mut section = text.sections.get_mut(0).unwrap();
        section.value = format!("Page {}", page);

        if let Ok(page) = page.parse::<usize>() {
            const PAGE: usize = 5 * 4;
            let offset = page * PAGE;
            let saves = (offset..offset + PAGE)
                .map(|i| saves.saves.get(&i))
                .collect::<Vec<_>>();

            for ((i, ui), save) in self.save_frames.iter_mut()
                .enumerate().zip(saves) {
                if let Some(header) = text_query
                    .get_mut(ui.num).unwrap()
                    .sections.get_mut(0) {
                    header.value = (offset + i).to_string();
                }
                ui.has_save = save.is_some();
                let mut hint = String::new();
                let mut btn_back = UiImage::default();
                let mut has_back = false;
                if let Some(save) = save {
                    hint = save.timestamp.format("%d/%m/%Y %H:%M").to_string();
                    if let Some(back) = &save.state.inspector.background {
                        btn_back = asset_server.load(back).into();
                        has_back = true;
                    }
                }
                if let Some(section) = text_query
                    .get_mut(ui.hint).unwrap()
                    .sections.get_mut(0) {
                    section.value = hint;
                }
                if let Ok(mut img) = img_query.get_mut(ui.btn) {
                    *img = btn_back;
                }
                color_query.get_mut(ui.btn).unwrap().0 = if has_back {
                    Color::WHITE
                } else {
                    TRANSPARENT
                };
            }
        }
    }

    pub fn fix_save_mark(&self, mut mark: SaveMark) -> SaveMark {
        if let Ok(page) = self.current.parse::<usize>() {
            mark.to = mark.to + 20 * page;
            mark
        } else {
            unreachable!()
        }
    }

    pub fn fix_load_mark(&self, mut mark: LoadMark) -> LoadMark {
        if let Ok(page) = self.current.parse::<usize>() {
            mark.0 = mark.0 + 20 * page;
            mark
        } else {
            unreachable!()
        }
    }
}

pub struct SaveFrameUI {
    pub root: Entity,
    pub btn: Entity,

    pub has_save: bool,
    pub num: Entity,
    pub hint: Entity,
}

pub fn save_show(
    ui: Res<SaveLoadUI>,
    mut style_query: Query<&mut Style>,
)
{
    ui.show(&mut style_query);
}

pub struct NewPage(pub String);

pub fn save_page_actions(
    mut commands: Commands,
    mut page_query: Query<
        (&Interaction, &mut UiColor, &SavePageButton),
        (Changed<Interaction>, With<Button>),
    >,
)
{
    let span = span!(Level::WARN, "save_page_actions");
    let _enter = span.enter();

    for (interaction, mut color, btn) in page_query.iter_mut() {
        let interaction: &Interaction = interaction;
        let btn: &SavePageButton = btn;
        match interaction {
            Interaction::Clicked => {
                *color = TRANSPARENT.into();
                commands.insert_resource(NewPage(btn.0.clone()));
            }
            Interaction::Hovered => {
                *color = Color::WHITE.into();
            }
            Interaction::None => {
                *color = TRANSPARENT.into();
            }
        }
    }
}

pub fn new_page(
    mut commands: Commands,
    event: Option<Res<NewPage>>,
    mut text_query: Query<&mut Text>,
    mut img_query: Query<&mut UiImage>,
    mut color_query: Query<&mut UiColor>,
    saves: Res<Saves>,
    asset_server: Res<AssetServer>,
    mut save_ui: ResMut<SaveLoadUI>,
)
{
    if let Some(page) = event {
        commands.remove_resource::<NewPage>();
        save_ui.select_page(
            &page.0,
            &mut text_query,
            &mut img_query,
            &mut color_query,
            saves.as_ref(),
            asset_server.as_ref(),
        );
    }
}

pub fn save_actions(
    mut commands: Commands,
    mut save_ui: ResMut<SaveLoadUI>,
    mut interactions_query: Query<
        (&Interaction, &mut UiColor, &SaveMark),
        (Changed<Interaction>, With<Button>),
    >,
)
{
    let span = span!(Level::WARN, "save_actions");
    let _enter = span.enter();

    for (interaction, mut color, mark) in interactions_query.iter_mut() {
        let interaction: &Interaction = interaction;
        match interaction {
            Interaction::Clicked => {
                *color = TRANSPARENT.into();
                commands.insert_resource(save_ui.fix_save_mark(mark.clone()));
                commands.insert_resource(NewPage(save_ui.current.clone()));
            }
            Interaction::Hovered => {
                *color = Color::rgba(1.0, 1.0, 1.0, 0.01).into();
            }
            Interaction::None => {
                *color = TRANSPARENT.into();
            }
        }
    }
}

pub fn load_actions(
    mut commands: Commands,
    mut save_ui: ResMut<SaveLoadUI>,
    mut interactions_query: Query<
        (&Interaction, &mut UiColor, &LoadMark),
        (Changed<Interaction>, With<Button>),
    >,
    mut ui_state: ResMut<State<UiState>>,
)
{
    let span = span!(Level::WARN, "load_actions");
    let _enter = span.enter();

    for (interaction, mut color, mark) in interactions_query.iter_mut() {
        let interaction: &Interaction = interaction;
        match interaction {
            Interaction::Clicked => {
                *color = TRANSPARENT.into();
                commands.insert_resource(save_ui.fix_load_mark(mark.clone()));
            }
            Interaction::Hovered => {
                *color = Color::rgba(1.0, 1.0, 1.0, 0.01).into();
            }
            Interaction::None => {
                *color = TRANSPARENT.into();
            }
        }
    }
}

pub fn save_hide(
    ui: Res<SaveLoadUI>,
    mut style_query: Query<&mut Style>,
)
{
    ui.hide(&mut style_query);
}

