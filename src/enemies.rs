use bevy::prelude::*;
use {
    crate::{
        camera::InGameCamera,
        constants::{
            CAR_EXPLOSION_SHAKE_AMOUNT, ENEMY_ACCELLERATION, ENEMY_MAX_SPEED, ENEMY_ROTATION_SPEED,
            RESOLUTION,
        },
        jerry_cans::spawn_jerry_can,
        player::{spawn_bullets, Bullet, PlayerStats, Velocity},
    },
    rand::{thread_rng, Rng},
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

pub fn setup_enemy_spawn_timer(commands: &mut Commands) {
    commands.spawn(EnemySpawnTimer(Timer::new(
        Duration::from_secs(3),
        TimerMode::Once,
    )));
}

pub fn spawn_enemy(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<&mut EnemySpawnTimer>,
    player_query: Query<&Transform, With<PlayerStats>>,
    asset_server: Res<AssetServer>,
) {
    let player_transform = match player_query.get_single() {
        Ok(value) => value,
        Err(_) => return,
    };

    let mut timer = match query.get_single_mut() {
        Ok(value) => value,
        Err(_) => return,
    };

    timer.0.tick(time.delta());

    let new_duration = timer.0.duration().as_secs_f32() * 0.9;

    if timer.0.just_finished() {
        let new_duration = Duration::from_secs_f32(if new_duration < 0.5 {
            1.0
        } else {
            new_duration
        });
        timer.0.set_duration(new_duration);
        timer.0.reset();

        let enemy_texture = asset_server.load("graphics/enemy.png");
        let vec2 = Vec2::from_angle(thread_rng().gen_range(-TAU..TAU)) * RESOLUTION.width as f32;

        commands.spawn(EnemyBundle {
            sprite_bundle: SpriteBundle {
                texture: enemy_texture,
                transform: Transform::from_xyz(
                    player_transform.translation.x + vec2.x,
                    player_transform.translation.y + vec2.y,
                    1.0,
                ),
                ..default()
            },
            velocity: Velocity(Vec2::ZERO),
            enemy: Enemy,
        });
    }
}

pub fn move_enemies(
    time: Res<Time>,
    player_query: Query<&Transform, (With<PlayerStats>, Without<Enemy>)>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Enemy>>,
) {
    let player_transform = match player_query.get_single() {
        Ok(value) => value,
        Err(_) => return,
    };

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

pub fn collide_with_enemies(
    mut commands: Commands,
    enemies: Query<(Entity, &Transform), (With<Enemy>, Without<Bullet>)>,
    bullets: Query<&Transform, With<Bullet>>,
    mut camera_query: Query<&mut InGameCamera>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for (enemy_entity, enemy_transform) in enemies.iter() {
        for bullet_transform in bullets.iter() {
            if enemy_transform
                .translation
                .distance(bullet_transform.translation)
                < 4.0 + bullet_transform.scale.length()
            {
                let mut camera = camera_query.single_mut();

                camera.screen_shake_multiplier = CAR_EXPLOSION_SHAKE_AMOUNT;

                commands.entity(enemy_entity).despawn();
                spawn_bullets(
                    45,
                    enemy_transform.clone(),
                    None,
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                );
                spawn_jerry_can(
                    enemy_transform.translation,
                    &mut commands,
                    &asset_server,
                    &mut texture_atlas_layouts,
                );
                break;
            }
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

pub fn get_angle(rotation: Quat) -> f32 {
    let axis_angle = rotation.to_axis_angle();
    return axis_angle.1 * axis_angle.0.z;
}
