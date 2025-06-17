use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins).add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single().unwrap();
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb_u8(11, 35, 35)),
            ..default()
        },
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 1.0),
    ));
}
