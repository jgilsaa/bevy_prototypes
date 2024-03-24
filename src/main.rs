use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy prototypes".to_string(),
                    ..default()
                }),
                ..default()
            }),
            WorldInspectorPlugin::default(),
        ))
        .run();
}
