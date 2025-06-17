use super::{container_node, spawn_button, spawn_text};
use crate::app_state::AppState;
use crate::game::Score;
use bevy::prelude::*;

pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameOver), spawn_game_over_menu)
            .add_systems(OnExit(AppState::GameOver), despawn_game_over_menu)
            .add_systems(
                Update,
                (
                    interact_with_main_menu_button,
                    interact_with_restart_button,
                    interact_with_quit_button,
                ),
            );
    }
}

#[derive(Component)]
pub struct GameOverMenu;

#[derive(Component)]
pub struct RestartButton;

#[derive(Component)]
pub struct MainMenuButton;

#[derive(Component)]
pub struct QuitButton;

fn spawn_game_over_menu(mut commands: Commands, asset_server: Res<AssetServer>, score: Res<Score>) {
    commands
        .spawn((container_node(), GameOverMenu {}))
        .with_children(|parent| {
            spawn_text(parent, &asset_server, "Game Over", 48.0);
            spawn_text(parent, &asset_server, &format!("Score: {}", score.value), 24.0);
            spawn_button(parent, &asset_server, "Main Menu", MainMenuButton {});
            spawn_button(parent, &asset_server, "Restart", RestartButton {});
            spawn_button(parent, &asset_server, "Quit", QuitButton {});
        });
}

fn despawn_game_over_menu(mut commands: Commands, query: Query<Entity, With<GameOverMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn interact_with_main_menu_button(
    mut button_query: Query<&Interaction, (Changed<Interaction>, With<MainMenuButton>)>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if let Ok(interaction) = button_query.single_mut() {
        if *interaction == Interaction::Pressed {
            next_app_state.set(AppState::MainMenu);
        }
    }
}

fn interact_with_restart_button(
    mut button_query: Query<&Interaction, (Changed<Interaction>, With<RestartButton>)>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if let Ok(interaction) = button_query.single_mut() {
        if *interaction == Interaction::Pressed {
            next_app_state.set(AppState::Game);
        }
    }
}

fn interact_with_quit_button(
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut button_query: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>,
) {
    if let Ok(interaction) = button_query.single_mut() {
        if *interaction == Interaction::Pressed {
            app_exit_event_writer.write(AppExit::Success);
        }
    }
}
