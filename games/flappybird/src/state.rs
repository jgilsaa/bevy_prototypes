use bevy::prelude::*;

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    #[default]
    Menu,
    InGame,
    Dead,
}
