use bevy::prelude::*;

use crate::{game_assets::GameAssets, DespawnOnReset, WINDOW_HEIGHT};

use super::component::Bird;

pub fn spawn_bird(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands.spawn((
        SpriteBundle {
            texture: game_assets.bird_mid_handle.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 2.0),
            ..default()
        },
        Bird::default(),
        DespawnOnReset,
        Name::from("Bird"),
    ));
}

pub fn flap(mut query: Query<&mut Bird>) {
    let mut bird = query.get_single_mut().unwrap();
    bird.velocity += 1.5;
    bird.velocity = bird.velocity.min(200.0);
}

pub fn fall(time: Res<Time>, mut query: Query<&mut Bird>) {
    let mut bird = query.get_single_mut().unwrap();
    bird.velocity -= bird.gravity * time.delta_seconds();
    bird.velocity = bird.velocity.max(-2.0);
}

pub fn apply_velocity(time: Res<Time>, mut query: Query<(&mut Transform, &Bird), With<Bird>>) {
    let (mut transform, bird) = query.get_single_mut().unwrap();
    transform.translation.y += bird.velocity * 200.0 * time.delta_seconds();
    transform.translation.y = transform.translation.y.min(WINDOW_HEIGHT / 2.0);
}
