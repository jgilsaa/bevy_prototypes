use bevy::prelude::*;

use crate::state::GameState;

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub struct GameoverMenu;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), spawn_menu)
            .add_systems(OnExit(GameState::Menu), despawn_menu)
            .add_systems(Update, start_on_spacebar.run_if(in_state(GameState::Menu)))
            .add_systems(OnEnter(GameState::Dead), spawn_gameover);
    }
}

fn spawn_menu(mut commands: Commands) {
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
            MainMenu,
            Name::from("Main menu"),
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section("Press <spacebar> to start", TextStyle { ..default() })
                    .with_style(Style {
                        position_type: PositionType::Absolute,
                        top: Val::Percent(20.),
                        ..default()
                    }),
            );
        });
}

fn despawn_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    for menu_entity in query.iter() {
        commands.entity(menu_entity).despawn_recursive();
    }
}

fn spawn_gameover(mut commands: Commands) {
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
            Name::from("Gameover"),
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "You died! \n Press <spacebar> to start",
                    TextStyle { ..default() },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(20.),
                    ..default()
                }),
            );
        });
}

fn start_on_spacebar(
    input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(KeyCode::Space) {
        game_state.set(GameState::InGame);
    }
}
