use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

use super::player::{PLAYER_SIZE, Player};
use super::state::running;
use crate::app_state::{AppState, GameOver};

const NUMBER_OF_ENEMIES: usize = 4;
const ENEMY_SPEED: f32 = 200.0;
const ENEMY_SIZE: f32 = 40.0;
const ENEMY_SPAWN_TIME: f32 = 3.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_systems(OnEnter(AppState::Game), spawn_enemies)
            .add_systems(OnExit(AppState::Game), despawn_enemies)
            //.add_systems(Startup, spawn_enemies)
            .add_systems(
                Update,
                (
                    tick_enemy_spawn_timer,
                    spawn_enemies_over_time,
                    enemy_movement,
                    update_enemy_direction,
                    enemy_hit_player,
                )
                    .run_if(running),
            );
    }
}

#[derive(Component)]
struct Enemy {
    direction: Vec2,
}

#[derive(Resource)]
struct EnemySpawnTimer {
    timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = window_query.single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        spawn_enemy(&mut commands, &window, &mut meshes, &mut materials);
    }
}

fn spawn_enemy(
    commands: &mut Commands,
    window: &Window,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let x = ENEMY_SIZE + random::<f32>() * (window.width() - (2.0 * ENEMY_SIZE));
    let y = ENEMY_SIZE + random::<f32>() * (window.height() - (2.0 * ENEMY_SIZE));

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(ENEMY_SIZE / 2.0)).into()),
        MeshMaterial2d(materials.add(Color::srgb_u8(255, 88, 88))),
        Transform::from_xyz(x, y, 0.0),
        Enemy {
            direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
        },
    ));
}

fn despawn_enemies(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for entity in enemy_query.iter() {
        commands.entity(entity).despawn();
    }
}

fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction: Vec3 = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_secs();
    }
}

fn update_enemy_direction(
    mut commands: Commands,
    mut enemy_query: Query<(&mut Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0;

    let x_min = half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for (mut transform, mut enemy) in enemy_query.iter_mut() {
        let mut translation = transform.translation;
        let mut direction_changed: bool = false;

        if translation.x < x_min {
            translation.x = x_min;
            enemy.direction.x *= -1.0;
            direction_changed = true
        }

        if translation.x > x_max {
            translation.x = x_max;
            enemy.direction.x *= -1.0;
            direction_changed = true
        }

        if translation.y < y_min {
            translation.y = y_min;
            enemy.direction.y *= -1.0;
            direction_changed = true
        }

        if translation.y > y_max {
            translation.y = y_max;
            enemy.direction.y *= -1.0;
            direction_changed = true
        }

        if direction_changed {
            transform.translation = translation;
            commands.spawn(AudioPlayer::<AudioSource>(asset_server.load("audio/pluck_001.ogg")));
        }
    }
}

fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((player_entity, player_transform)) = player_query.single_mut() {
        let min_distance = PLAYER_SIZE / 2.0 + ENEMY_SIZE / 2.0;

        for enemy_transform in enemy_query.iter() {
            if hits(&player_transform, &enemy_transform, min_distance) {
                commands.spawn(AudioPlayer::<AudioSource>(
                    asset_server.load("audio/explosionCrunch_000.ogg"),
                ));

                commands.entity(player_entity).despawn();
                game_over_event_writer.write(GameOver {});
            }
        }
    }
}

fn hits(transform1: &Transform, transform2: &Transform, min_distance: f32) -> bool {
    let distance = transform1.translation.distance(transform2.translation);
    distance < min_distance
}

fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    enemy_spawn_timer: ResMut<EnemySpawnTimer>,
) {
    if enemy_spawn_timer.timer.finished() {
        let window = window_query.single().unwrap();
        spawn_enemy(&mut commands, &window, &mut meshes, &mut materials);
    }
}
