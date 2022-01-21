use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Aerugo editor".into(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .run();
}