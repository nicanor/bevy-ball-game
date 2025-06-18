use super::stars::CollectStarEvent;
use crate::app_state::AppState;
use crate::ui::{box_node, hub_node, text_bundle};
use bevy::prelude::*;

pub struct HubPlugin;

impl Plugin for HubPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_hub)
            .add_systems(OnExit(AppState::Game), despawn_hub)
            .add_observer(collect_star);
    }
}

#[derive(Component)]
pub struct Hub;

#[derive(Component)]
pub struct ScoreDisplay;

fn spawn_hub(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Hub,
        hub_node(),
        children![(
            box_node(),
            children![
                text_bundle(&asset_server, "Score: ", 48.0),
                (ScoreDisplay, text_bundle(&asset_server, "0", 48.0))
            ]
        )],
    ));
}

fn despawn_hub(mut commands: Commands, query: Query<Entity, With<Hub>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn collect_star(_trigger: Trigger<CollectStarEvent>, mut query: Query<&mut Text, With<ScoreDisplay>>) {
    for mut text in query.iter_mut() {
        let score = text.parse::<u32>().unwrap();
        **text = (score + 1).to_string();
    }
}
