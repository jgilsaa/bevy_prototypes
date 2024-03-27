use bevy::prelude::*;

use crate::state::GameState;

use self::{
    components::PipeSpawner,
    systems::{check_passed_pipes, despawn_out_of_bounds_pipes, scroll_pipes, spawn_pipes},
};

mod components;
mod systems;

pub struct PipesPlugin;

impl Plugin for PipesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PipeSpawner::default()).add_systems(
            Update,
            (
                spawn_pipes,
                despawn_out_of_bounds_pipes,
                scroll_pipes,
                check_passed_pipes,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}
