use super::player::{PLAYER_SIZE, Player};
use super::score::Score;
use super::state::running;
use crate::app_state::AppState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

const STAR_COUNT: usize = 10;
const STAR_SIZE: f32 = 20.0;
const STAR_SPAWN_TIME: f32 = 1.0;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_event::<CollectStarEvent>()
            .add_systems(OnEnter(AppState::Game), spawn_stars)
            .add_systems(OnExit(AppState::Game), unspawn_stars)
            .add_systems(
                Update,
                (collect_star, tick_spawn_timer, spawn_stars_over_time).run_if(running),
            );
    }
}

#[derive(Component)]
struct Star {}

#[derive(Resource)]
struct StarSpawnTimer {
    timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

#[derive(Event)]
pub struct CollectStarEvent;

fn spawn_star(
    commands: &mut Commands,
    window: &Window,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let x = random::<f32>() * (window.width() - STAR_SIZE);
    let y = random::<f32>() * (window.height() - STAR_SIZE);

    commands.spawn((
        Mesh2d(meshes.add(RegularPolygon::new(STAR_SIZE / 2.0, 5)).into()),
        MeshMaterial2d(materials.add(Color::srgb_u8(190, 243, 84))),
        Transform::from_xyz(x, y, 0.0),
        Star {},
    ));
}

fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for _ in 0..STAR_COUNT {
        let window = window_query.single().unwrap();
        spawn_star(&mut commands, &window, &mut meshes, &mut materials);
    }
}

fn unspawn_stars(mut commands: Commands, star_query: Query<Entity, With<Star>>) {
    for entity in star_query.iter() {
        commands.entity(entity).despawn();
    }
}

fn collect_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.single() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform.translation.distance(star_transform.translation);

            let hit_distance: f32 = PLAYER_SIZE / 2.0 + STAR_SIZE / 2.0;

            if distance < hit_distance {
                score.value += 1;
                let sound_effect = asset_server.load("audio/laserLarge_000.ogg");

                commands.spawn(AudioPlayer::<AudioSource>(sound_effect));

                // commands.spawn(AudioPlayer {
                //     settings: PlaybackSettings {
                //         mode: PlaybackMode::Despawn,
                //         ..default()
                //     },
                //     source: sound_effect,
                //     ..default()
                // });

                commands.trigger(CollectStarEvent);

                commands.entity(star_entity).despawn();
            }
        }
    }
}

fn tick_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    star_spawn_timer: ResMut<StarSpawnTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.single().unwrap();
        spawn_star(&mut commands, &window, &mut meshes, &mut materials);
    }
}
