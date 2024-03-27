use bevy::prelude::*;

use crate::game_assets::GameAssets;

pub enum BirdAnimation {
    Up,
    Down,
    Mid,
}

impl BirdAnimation {
    pub fn next(animation: BirdAnimation) -> BirdAnimation {
        match animation {
            BirdAnimation::Down => BirdAnimation::Mid,
            BirdAnimation::Mid => BirdAnimation::Up,
            BirdAnimation::Up => BirdAnimation::Down,
        }
    }

    pub fn to_image(animation: BirdAnimation, game_assets: &GameAssets) -> Handle<Image> {
        match animation {
            BirdAnimation::Down => game_assets.bird_down_handle.clone(),
            BirdAnimation::Mid => game_assets.bird_mid_handle.clone(),
            BirdAnimation::Up => game_assets.bird_up_handle.clone(),
        }
    }
}
