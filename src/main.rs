mod app_state;
mod game;
mod setup;
mod ui;

use app_state::AppStatePlugin;
use bevy::prelude::*;
use game::GamePlugin;
use setup::SetupPlugin;
use ui::UIPlugin;

fn main() {
    App::new()
        .add_plugins(SetupPlugin)
        .add_plugins(AppStatePlugin)
        .add_plugins(UIPlugin)
        .add_plugins(GamePlugin)
        .run();
}
