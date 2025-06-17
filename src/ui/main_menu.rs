use super::{container_node, spawn_button, spawn_title};
use crate::app_state::AppState;
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu)
            .add_systems(Update, (interact_with_play_button, interact_with_quit_button));
    }
}

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct QuitButton;

fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((container_node(), MainMenu {})).with_children(|parent| {
        spawn_title(parent, &asset_server);
        spawn_button(parent, &asset_server, "Play", PlayButton {});
        spawn_button(parent, &asset_server, "Quit", QuitButton {});
    });
}

fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    for entity in main_menu_query.iter() {
        commands.entity(entity).despawn();
    }
}

fn interact_with_play_button(
    mut button_query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
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
