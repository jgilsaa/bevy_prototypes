use bevy::prelude::*;
use rand::Rng;

use crate::{
    bird::component::Bird, collision::Collider, game_assets::GameAssets, DespawnOnReset, PIPE_GAP,
    PIPE_SIZE, WINDOW_WIDTH,
};

use super::components::{ApproachingPipe, Pipe, PipeSpawner};

pub fn spawn_pipes(
    mut commands: Commands,
    time: Res<Time>,
    game_assets: Res<GameAssets>,
    mut pipe_spawner: ResMut<PipeSpawner>,
) {
    pipe_spawner.timer.tick(time.delta());
    if !pipe_spawner.timer.finished() {
        return;
    }

    let mut rng = rand::thread_rng();
    let y = rng.gen_range(-60.0..60.0);
    let x = WINDOW_WIDTH / 2.0;

    commands.spawn((
        SpriteBundle {
            texture: game_assets.pipe.clone(),
            transform: Transform::from_xyz(x, y - 160.0, 0.5),
            ..default()
        },
        Pipe,
        Collider { size: PIPE_SIZE },
        DespawnOnReset,
        Name::from("Pipe"),
    ));

    commands.spawn((
        SpriteBundle {
            texture: game_assets.pipe.clone(),
            transform: Transform::from_xyz(x, y + 160.0 + PIPE_GAP, 0.5),
            sprite: Sprite {
                flip_y: true,
                ..default()
            },
            ..default()
        },
        Pipe,
        Collider { size: PIPE_SIZE },
        DespawnOnReset,
        Name::from("Pipe"),
    ));
}

pub fn despawn_out_of_bounds_pipes(
    mut commands: Commands,
    pipes_query: Query<(Entity, &Transform), With<Pipe>>,
) {
    pipes_query
        .iter()
        .filter(|(_entity, transform)| transform.translation.x <= -(WINDOW_WIDTH / 2.0))
        .for_each(|(entity, _transform)| commands.entity(entity).despawn());
}

pub fn scroll_pipes(mut pipes_query: Query<&mut Transform, With<Pipe>>, time: Res<Time>) {
    for mut transform in pipes_query.iter_mut() {
        transform.translation.x -= 150.0 * time.delta_seconds();
    }
}

pub fn check_passed_pipes(
    mut commands: Commands,
    bird_query: Query<&Transform, With<Bird>>,
    pipes_query: Query<(Entity, &Transform), With<ApproachingPipe>>,
) {
    let bird_transform = bird_query.get_single().unwrap();
    pipes_query
        .iter()
        .filter(|(_, transform)| transform.translation.x < bird_transform.translation.x)
        .for_each(|(entity, _)| {
            commands.entity(entity).remove::<ApproachingPipe>();
            // TODO: Add score increment
        });
}
