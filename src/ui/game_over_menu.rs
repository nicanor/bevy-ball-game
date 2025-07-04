use super::{button_bundle, container_node, text_bundle};
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
struct GameOverMenu;

#[derive(Component)]
struct RestartButton;

#[derive(Component)]
struct MainMenuButton;

#[derive(Component)]
struct QuitButton;

fn spawn_game_over_menu(mut commands: Commands, asset_server: Res<AssetServer>, score: Res<Score>) {
    commands.spawn((
        GameOverMenu,
        container_node(),
        children![
            text_bundle(&asset_server, "Game Over", 48.0),
            text_bundle(&asset_server, &format!("Score: {}", score.value), 24.0),
            button_bundle(&asset_server, "Main Menu", MainMenuButton),
            button_bundle(&asset_server, "Restart", RestartButton),
            button_bundle(&asset_server, "Quit", QuitButton),
        ],
    ));
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
