use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    speed: f32,
}

pub const PLAYER_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
const PLAYER_SPEED: f32 = 300.;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_systems(Startup, spawn_player)
            .add_systems(FixedUpdate, player_movement);
    }
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            // transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 2.),
                scale: PLAYER_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(0.3, 0.3, 0.7),
                ..default()
            },
            ..default()
        })
        .insert(Player {
            speed: PLAYER_SPEED,
        });
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &Player)>,
    time: Res<Time>,
) {
    let (mut player_transform, player) = player_query.single_mut();

    let mut direction = Vec2::ZERO;

    for input in keyboard_input.get_pressed() {
        match *input {
            KeyCode::W => direction.y += 1.,
            KeyCode::A => direction.x -= 1.,
            KeyCode::S => direction.y -= 1.,
            KeyCode::D => direction.x += 1.,
            _ => {}
        }
    }

    let movement = direction.normalize_or_zero().extend(0.) * player.speed * time.delta_seconds();

    player_transform.translation += movement;
}
