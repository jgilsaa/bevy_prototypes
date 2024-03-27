use bevy::prelude::*;

use crate::{has_user_input, state::GameState};

use self::systems::{apply_velocity, fall, flap, spawn_bird};

pub mod animation;
pub mod component;
pub mod systems;

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_bird)
            .add_systems(
                Update,
                (flap.run_if(has_user_input), fall, apply_velocity)
                    .run_if(in_state(GameState::InGame)),
            );
    }
}
