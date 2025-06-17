use bevy::prelude::*;
use enemies::EnemyPlugin;
use hub::HubPlugin;
use pause_menu::PauseMenuPlugin;
use player::PlayerPlugin;
pub use score::Score;
use score::ScorePlugin;
use stars::StarPlugin;
use state::GameStatePlugin;

mod enemies;
mod hub;
mod pause_menu;
mod player;
mod score;
mod stars;
mod state;
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GameStatePlugin)
            .add_plugins(HubPlugin)
            .add_plugins(PauseMenuPlugin)
            .add_plugins(ScorePlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(StarPlugin)
            .add_plugins(EnemyPlugin);
    }
}
