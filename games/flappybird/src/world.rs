use bevy::prelude::*;

use crate::{collision::Collider, WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_background);
    }
}

fn spawn_background(mut commands: Commands, assets_server: Res<AssetServer>) {
    let background_image = assets_server.load("background-day.png");
    let floor_image = assets_server.load("base.png");

    commands.spawn((
        SpriteBundle {
            texture: background_image,
            ..default()
        },
        Name::from("Background"),
    ));

    commands.spawn((
        SpriteBundle {
            texture: floor_image,
            sprite: Sprite {
                custom_size: Some(Vec2::new(WINDOW_WIDTH, 112.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -(WINDOW_HEIGHT / 2.0), 1.0),
            ..default()
        },
        Collider {
            size: Vec2::new(WINDOW_WIDTH, 112.0),
        },
        Name::from("Floor"),
    ));
}
