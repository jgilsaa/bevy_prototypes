use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};

use crate::{bird::component::Bird, state::GameState, BIRD_SIZE};

#[derive(Component)]
pub struct Collider {
    pub size: Vec2,
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (check_collision).run_if(in_state(GameState::InGame)),
        );
    }
}

fn check_collision(
    collider_query: Query<(&Transform, &Collider), With<Collider>>,
    bird_query: Query<&Transform, With<Bird>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for bird_transform in bird_query.iter() {
        for (obj_transform, collider) in collider_query.iter() {
            let collision =
                Aabb2d::new(bird_transform.translation.xy(), BIRD_SIZE / 2.0).intersects(
                    &Aabb2d::new(obj_transform.translation.xy(), collider.size / 2.0),
                );

            if collision {
                game_state.set(GameState::Dead);
            }
        }
    }
}
