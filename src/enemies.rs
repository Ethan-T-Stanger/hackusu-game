use bevy::prelude::*;
use {
    crate::{
        constants::{ENEMY_ACCELLERATION, ENEMY_MAX_SPEED, ENEMY_ROTATION_SPEED, ROTATION_SPEED},
        player::{PlayerGun, Velocity},
    },
    std::{f32::consts::TAU, time::Duration},
};

#[derive(Bundle)]
pub struct EnemyBundle {
    sprite_bundle: SpriteBundle,
    velocity: Velocity,
    enemy: Enemy,
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct EnemySpawnTimer(Timer);

pub fn setup_enemy_spawn_timer(mut commands: Commands) {
    commands.spawn(EnemySpawnTimer(Timer::new(
        Duration::from_secs(5),
        TimerMode::Once,
    )));
}

pub fn spawn_enemy(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<&mut EnemySpawnTimer>,
    asset_server: Res<AssetServer>,
) {
    let mut timer = query.single_mut();

    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        let new_duration =
            Duration::from_millis((timer.0.duration().as_millis() as f32 * 0.99).floor() as u64);
        timer.0.set_duration(new_duration);
        timer.0.reset();

        let enemy_texture = asset_server.load("graphics/enemy.png");

        commands.spawn(EnemyBundle {
            sprite_bundle: SpriteBundle {
                texture: enemy_texture,
                transform: Transform::from_xyz(10.0, 30.0, 1.0),
                ..default()
            },
            velocity: Velocity(Vec2::ZERO),
            enemy: Enemy,
        });
    }
}

pub fn move_enemies(
    time: Res<Time>,
    player_query: Query<&Transform, (With<PlayerGun>, Without<Enemy>)>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Enemy>>,
) {
    let player_transform = player_query.single();

    for (mut enemy, mut velocity) in query.iter_mut() {
        let enemy_angle = get_angle(enemy.rotation);
        let angle_to_player = (Vec2::new(enemy.translation.x, enemy.translation.y)
            - Vec2::new(
                player_transform.translation.x,
                player_transform.translation.y,
            ))
        .to_angle();

        if fix_angle(angle_to_player, Some(enemy_angle)) < enemy_angle {
            enemy.rotate_z(ENEMY_ROTATION_SPEED * time.delta_seconds());
        }
        if fix_angle(angle_to_player, Some(enemy_angle)) > enemy_angle {
            enemy.rotate_z(-ENEMY_ROTATION_SPEED * time.delta_seconds());
        }

        velocity.0 += Vec2::from_angle(enemy_angle) * ENEMY_ACCELLERATION;

        if velocity.0.length() > ENEMY_MAX_SPEED {
            velocity.0 = velocity.0.normalize() * ENEMY_MAX_SPEED;
        }
    }
}

fn fix_angle(angle: f32, bounding_angle: Option<f32>) -> f32 {
    let mut temp_angle = angle;

    while temp_angle < -TAU / 2.0 + bounding_angle.unwrap_or(0.0) {
        temp_angle += TAU
    }
    while temp_angle > TAU / 2.0 + bounding_angle.unwrap_or(0.0) {
        temp_angle -= TAU
    }

    return temp_angle;
}

fn get_angle(rotation: Quat) -> f32 {
    let axis_angle = rotation.to_axis_angle();
    return axis_angle.1 * axis_angle.0.z;
}