use bevy::prelude::*;

use crate::{has_user_input, state::GameState, DespawnOnReset};

#[derive(Component)]
pub struct GameoverMenu;

pub struct GameoverPlugin;

impl Plugin for GameoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Dead), spawn_gameover_menu)
            .add_systems(
                Update,
                reset.run_if(in_state(GameState::Dead).and_then(has_user_input)),
            );
    }
}

fn reset(
    mut commands: Commands,
    query: Query<Entity, With<DespawnOnReset>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    query
        .iter()
        .for_each(|entity| commands.entity(entity).despawn_recursive());

    game_state.set(GameState::Menu);
}

fn spawn_gameover_menu(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_content: AlignContent::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            GameoverMenu,
            DespawnOnReset,
            Name::from("Gameover"),
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section("You died!", TextStyle { ..default() }).with_style(
                    Style {
                        position_type: PositionType::Absolute,
                        top: Val::Percent(20.),
                        ..default()
                    },
                ),
            );

            parent.spawn(
                TextBundle::from_section(
                    "Press <spacebar> to try again",
                    TextStyle { ..default() },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(30.),
                    ..default()
                }),
            );
        });
}
