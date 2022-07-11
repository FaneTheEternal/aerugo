use std::io::Read;
use bevy::ecs::schedule::{ShouldRun};
use bevy::prelude::*;
use aerugo::Aerugo;

pub const BTN_NORMAL: Color = Color::WHITE;
pub const BTN_HOVERED: Color = Color::GRAY;
pub const BTN_PRESSED: Color = Color::DARK_GRAY;

pub const TRANSPARENT: Color = Color::rgba(1.0, 1.0, 1.0, 0.0);
pub const GLASS_RED: Color = Color::rgba(1.0, 0.0, 0.0, 0.5);

pub const Z_SCENE: f32 = 15.0;
pub const Z_BACKGROUND: f32 = 5.0;
pub const Z_SPRITE: f32 = 10.0;
pub const Y_SPRITE: f32 = 0.0;

pub const SIZE_ALL: Size<Val> = Size { width: Val::Percent(100.0), height: Val::Percent(100.0) };

pub fn make_button_closure<B>(
    text: &str,
    font: Handle<Font>,
    button: B,
    button_color: Color,
) -> impl FnOnce(&mut ChildBuilder) + '_
    where B: Component
{
    move |parent| {
        parent
            .spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(200.0), Val::Px(60.0)),
                    margin: UiRect::all(Val::Px(20.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                color: button_color.into(),
                ..Default::default()
            })
            .insert(button)
            .with_children(|parent| {
                parent
                    .spawn_bundle(TextBundle {
                        text: Text::with_section(
                            text,
                            TextStyle {
                                font,
                                font_size: 40.0,
                                color: Color::BLACK,
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
            });
    }
}

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
