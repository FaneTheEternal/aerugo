use bevy::prelude::*;
use crate::utils::SIZE_ALL;
use super::*;

pub fn spawn_splash_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<State<MainState>>,
)
{
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: SIZE_ALL,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            color: Color::BLACK.into(),
            ..Default::default()
        })
        .insert(SplashScreen)
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text::from_section(
                        "Aerugo",
                        TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 50.0,
                            color: Color::WHITE,
                        },
                    ),
                    ..Default::default()
                });
        });
    state.set(MainState::Load).unwrap_or_else(|e| warn!("{e:?}"));
}

pub fn remove_splash_screen(
    mut commands: Commands,
    query: Query<Entity, With<SplashScreen>>,
    mut ui_state: ResMut<State<UiState>>,
)
{
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    ui_state.set(UiState::MainMenu).unwrap_or_else(|e| warn!("{e:?}"));
}

pub fn load(
    mut commands: Commands,
    mut state: ResMut<State<MainState>>,
)
{
    let aerugo = crate::utils::load_aerugo();
    let saves = pre_load_saves(&aerugo);
    commands.insert_resource(aerugo);
    commands.insert_resource(saves);
    state.set(MainState::Spawn).unwrap_or_else(|e| warn!("{e:?}"));
}
