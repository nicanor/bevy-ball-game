use super::state::running;
use crate::app_state::AppState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub const PLAYER_SIZE: f32 = 64.0;
const PLAYER_SPEED: f32 = 800.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_player)
            .add_systems(OnExit(AppState::Game), unspawn_player)
            .add_systems(Update, (player_movement, confine_player_movement).run_if(running));
    }
}

#[derive(Component)]
pub struct Player {}

fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = window_query.single().unwrap();
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(PLAYER_SIZE / 2.0)).into()),
        MeshMaterial2d(materials.add(Color::srgb_u8(106, 210, 210))),
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        Player {},
    ));
}

fn unspawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    for entity in player_query.iter() {
        commands.entity(entity).despawn();
    }
}

fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.single_mut() {
        let mut direction = Vec3::ZERO;

        match (
            keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]),
            keys.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]),
            keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]),
            keys.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]),
        ) {
            (true, false, false, false) => direction += Vec3::new(-1.0, 0.0, 0.0),
            (false, true, false, false) => direction += Vec3::new(0.0, -1.0, 0.0),
            (false, false, true, false) => direction += Vec3::new(1.0, 0.0, 0.0),
            (false, false, false, true) => direction += Vec3::new(0.0, 1.0, 0.0),
            (true, true, false, false) => direction += Vec3::new(-0.7142, -0.7142, 0.0),
            (true, false, false, true) => direction += Vec3::new(-0.7142, 0.7142, 0.0),
            (false, true, true, false) => direction += Vec3::new(0.7142, -0.7142, 0.0),
            (false, false, true, true) => direction += Vec3::new(0.7142, 0.7142, 0.0),
            _ => {}
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_secs();
    }
}

fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.single_mut() {
        let window = window_query.single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0;

        let x_min = half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }
        player_transform.translation = translation;
    }
}
