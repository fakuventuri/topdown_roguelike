use bevy::prelude::*;
use rand::{
    distributions::{Distribution, Standard, Uniform},
    Rng,
};

use crate::CameraMarker;

use super::player::Player;

pub struct EnemyPlugin;

#[derive(Component)]
struct Enemy {
    speed: f32,
}

#[derive(Resource)]
struct EnemyCount(u32);

const MAX_ENEMY_COUNT: u32 = 50;
const ENEMY_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
const ENEMY_SPEED: f32 = 200.;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app //
            .insert_resource(EnemyCount(0))
            .add_systems(Update, spawn_enemies)
            .add_systems(FixedUpdate, move_towards_player);
    }
}

fn move_towards_player(
    mut enemies_query: Query<(&mut Transform, &Enemy)>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    let player_transform = player_query.single();

    let min_dist = player_transform.scale.xy().min_element();

    for (mut enemy_transform, enemy) in enemies_query.iter_mut() {
        let enemy_to_player = player_transform.translation.xy() - enemy_transform.translation.xy();

        if enemy_to_player.length() > min_dist {
            enemy_transform.translation +=
                enemy_to_player.normalize_or_zero().extend(0.) * enemy.speed * time.delta_seconds();
        }
    }
}

#[derive(Debug)]
enum ScreenBorders {
    Top,
    Bottom,
    Left,
    Right,
}

// impl ScreenBorders {
//     fn gen_rand_vec2_on_border<R: Rng + ?Sized>(
//         &self,
//         rng: &mut R,
//         area: Rect,
//         uniform_width: Uniform<f32>,
//         uniform_height: Uniform<f32>,
//     ) -> Vec2 {
//         match *self {
//             ScreenBorders::Top => Vec2::new(uniform_width.sample(rng), area.max.y),
//             ScreenBorders::Bottom => Vec2::new(uniform_width.sample(rng), area.min.y),
//             ScreenBorders::Left => Vec2::new(area.min.x, uniform_height.sample(rng)),
//             ScreenBorders::Right => Vec2::new(area.max.x, uniform_height.sample(rng)),
//         }
//     }
// }

impl Distribution<ScreenBorders> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ScreenBorders {
        match rng.gen_range(0..4) {
            0 => ScreenBorders::Top,
            1 => ScreenBorders::Bottom,
            2 => ScreenBorders::Left,
            _ => ScreenBorders::Right,
        }
    }
}

fn spawn_enemies(
    mut commands: Commands,
    mut enemy_count: ResMut<EnemyCount>,
    camera_query: Query<&OrthographicProjection, With<CameraMarker>>,
) {
    let camera_area = camera_query.single().area;
    let mut rng = rand::thread_rng();

    let uniform_width = Uniform::from(camera_area.min.x..camera_area.max.x);
    let uniform_height = Uniform::from(camera_area.min.y..camera_area.max.y);

    while enemy_count.0 < MAX_ENEMY_COUNT {
        let screen_border: ScreenBorders = rand::random();
        let translation_vec2 = match screen_border {
            ScreenBorders::Top => Vec2::new(
                uniform_width.sample(&mut rng),
                camera_area.max.y + ENEMY_SIZE.y / 2.,
            ),
            ScreenBorders::Bottom => Vec2::new(
                uniform_width.sample(&mut rng),
                camera_area.min.y - ENEMY_SIZE.y / 2.,
            ),
            ScreenBorders::Left => Vec2::new(
                camera_area.min.x - ENEMY_SIZE.x / 2.,
                uniform_height.sample(&mut rng),
            ),
            ScreenBorders::Right => Vec2::new(
                camera_area.max.x + ENEMY_SIZE.x / 2.,
                uniform_height.sample(&mut rng),
            ),
        };

        commands
            .spawn(SpriteBundle {
                // transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                transform: Transform {
                    translation: translation_vec2.extend(1.),
                    scale: ENEMY_SIZE,
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgb(1.0, 0.3, 0.3),
                    ..default()
                },
                ..default()
            })
            .insert(Enemy { speed: ENEMY_SPEED });

        enemy_count.0 += 1;
        // info!("count: {}", enemy_count.0);
    }
}
