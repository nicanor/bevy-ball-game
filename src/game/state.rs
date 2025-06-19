use crate::app_state::AppState;
use bevy::prelude::*;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(OnEnter(AppState::Game), resume)
            .add_systems(Update, toggle_state.run_if(in_state(AppState::Game)));
    }
}

pub fn running(game_state: Res<State<GameState>>, app_state: Res<State<AppState>>) -> bool {
    game_state.eq(&GameState::Running) && app_state.eq(&AppState::Game)
}

pub fn paused(game_state: Res<State<GameState>>, app_state: Res<State<AppState>>) -> bool {
    game_state.eq(&GameState::Paused) && app_state.eq(&AppState::Game)
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Running,
    Paused,
}

fn resume(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Running);
}

fn toggle_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    state: ResMut<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match state.get() {
            GameState::Paused => {
                next_state.set(GameState::Running);
            }
            GameState::Running => {
                next_state.set(GameState::Paused);
            }
        }
    }
}
