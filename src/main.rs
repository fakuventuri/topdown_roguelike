mod plugins;

use bevy::{prelude::*, render::camera::ScalingMode};
use plugins::{
    enemy::EnemyPlugin,
    player::{Player, PlayerPlugin},
};

#[derive(Component)]
struct CameraMarker;

const BACKGROUND_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);

fn main() {
    App::new() //
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        // mode: bevy::window::WindowMode::BorderlessFullscreen,
                        // resizable: false,
                        // resolution: WindowResolution::new(1920., 1080.),
                        title: "Roguelike".to_string(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                bevy::window::close_on_esc,
                toggle_fullscreen,
                camera_follow_player,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();

    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(720.0);

    commands.spawn((camera_bundle, CameraMarker));
}

fn camera_follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<CameraMarker>, Without<Player>)>,
) {
    camera_query.single_mut().translation = player_query.single().translation;
}

fn toggle_fullscreen(keyboard_input: Res<Input<KeyCode>>, mut windows: Query<&mut Window>) {
    if keyboard_input.just_pressed(KeyCode::F) {
        let mut window = windows.single_mut();
        match window.mode {
            bevy::window::WindowMode::Windowed => {
                window.mode = bevy::window::WindowMode::BorderlessFullscreen
            }
            bevy::window::WindowMode::BorderlessFullscreen => {
                window.mode = bevy::window::WindowMode::Windowed
            }
            _ => {}
        }
    }
}
