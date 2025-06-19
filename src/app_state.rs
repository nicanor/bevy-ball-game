use bevy::app::AppExit;
use bevy::prelude::*;

pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>().add_event::<GameOver>().add_systems(
            Update,
            (
                exit_event,
                handle_game_over,
                transition_to_game_state,
                transition_to_main_menu_state,
            ),
        );
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}

#[derive(Event)]
pub struct GameOver;

fn transition_to_game_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    state: ResMut<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        if !state.eq(&AppState::Game) {
            next_state.set(AppState::Game);
        }
    }
}

fn transition_to_main_menu_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    state: ResMut<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        if !state.eq(&AppState::MainMenu) {
            next_state.set(AppState::MainMenu);
        }
    }
}

fn exit_event(keyboard_input: Res<ButtonInput<KeyCode>>, mut event_writer: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        event_writer.write(AppExit::Success);
    }
}

fn handle_game_over(mut event_reader: EventReader<GameOver>, mut next_state: ResMut<NextState<AppState>>) {
    for _game_over_event in event_reader.read() {
        next_state.set(AppState::GameOver);
    }
}
