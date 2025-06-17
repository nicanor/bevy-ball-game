use super::state::GameState;
use super::state::paused;
use crate::app_state::AppState;
use crate::ui::{container_node, spawn_button, spawn_text};
use bevy::prelude::*;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Paused),
            spawn_pause_menu.run_if(in_state(AppState::Game)),
        )
        .add_systems(OnExit(GameState::Paused), despawn_pause_menu)
        .add_systems(OnExit(AppState::Game), despawn_pause_menu)
        .add_systems(
            Update,
            (
                interact_with_main_menu_button,
                interact_with_resume_button,
                interact_with_quit_button,
            )
                .run_if(paused),
        );
    }
}

#[derive(Component)]
pub struct PauseMenu;

#[derive(Component)]
pub struct ResumeButton;

#[derive(Component)]
pub struct MainMenuButton;

#[derive(Component)]
pub struct QuitButton;

fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((container_node(), PauseMenu {}))
        .with_children(|parent| {
            spawn_text(parent, &asset_server, "Paused", 48.0);
            spawn_button(parent, &asset_server, "Main Menu", MainMenuButton {});
            spawn_button(parent, &asset_server, "Resume", ResumeButton {});
            spawn_button(parent, &asset_server, "Quit", QuitButton {});
        });
}

fn despawn_pause_menu(mut commands: Commands, query: Query<Entity, With<PauseMenu>>) {
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

fn interact_with_resume_button(
    mut button_query: Query<&Interaction, (Changed<Interaction>, With<ResumeButton>)>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(interaction) = button_query.single_mut() {
        if *interaction == Interaction::Pressed {
            next_game_state.set(GameState::Running);
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
