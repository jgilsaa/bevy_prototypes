use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod bird;
mod collision;
mod game_assets;
mod gameover;
mod menu;
mod pipe;
mod state;
mod world;

use bird::BirdPlugin;
use collision::CollisionPlugin;
use game_assets::GameAssetsPlugin;
use gameover::GameoverPlugin;
use menu::MenuPlugin;
use pipe::PipesPlugin;
use state::GameState;
use world::WorldPlugin;

pub const WINDOW_WIDTH: f32 = 288.0;
pub const WINDOW_HEIGHT: f32 = 512.0;

pub const FLOOR_Z: f32 = 0.5;
pub const PIPE_Z: f32 = 1.0;
pub const BIRD_Z: f32 = 2.0;

pub const BIRD_SIZE: Vec2 = Vec2::new(34.0, 24.0);
pub const PIPE_SIZE: Vec2 = Vec2::new(52.0, 320.0);
pub const PIPE_GAP: f32 = 160.0;

#[derive(Component)]
pub struct DespawnOnReset;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Flappybird".to_string(),
                    // resolution: WindowResolution::new(288., 512.),
                    // resizable: false,
                    ..default()
                }),
                ..default()
            }),
            WorldInspectorPlugin::default(),
        ))
        .init_state::<GameState>()
        .add_plugins((
            GameAssetsPlugin,
            MenuPlugin,
            GameoverPlugin,
            WorldPlugin,
            BirdPlugin,
            PipesPlugin,
            CollisionPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn has_user_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    touch_input: Res<Touches>,
) -> bool {
    keyboard_input.just_pressed(KeyCode::Space)
        || mouse_button_input.just_pressed(MouseButton::Left)
        || touch_input.any_just_pressed()
}
