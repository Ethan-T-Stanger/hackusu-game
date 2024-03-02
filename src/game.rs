use bevy::prelude::*;
use {
    crate::constants::{
        BOOST_ACCELERATION_SPEED, BULLET_SPEED, BULLET_VELOCITY_OFFSET, DRAG, MAX_SPEED,
        PASSIVE_ACCELERATION_SPEED, ROTATION_SPEED,
    },
    rand::{thread_rng, Rng},
    std::{f32::consts::TAU, time::Duration},
};

#[derive(Bundle)]
pub struct Player {
    sprite_bundle: SpriteBundle,
    velocity: Velocity,
    player_gun: PlayerGun,
}

#[derive(Component)]
pub struct PlayerGun {
    shoot_timer: Timer,
    ammunition: u32,
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

pub fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Player {
        sprite_bundle: SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            texture: asset_server.load("graphics/player.png"),
            ..default()
        },
        velocity: Velocity(Vec2::ZERO),
        player_gun: PlayerGun {
            shoot_timer: Timer::new(Duration::from_millis(50), TimerMode::Once),
            ammunition: 100,
        },
    });
}

pub fn control_player(
    time: Res<Time>,
    commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut PlayerGun, &mut Transform, &mut Velocity)>,
) {
    let (mut player_gun, mut transform, mut velocity) = query.single_mut();

    if input.pressed(KeyCode::KeyA) {
        transform.rotate_z(ROTATION_SPEED * time.delta_seconds());
    }
    if input.pressed(KeyCode::KeyD) {
        transform.rotate_z(-ROTATION_SPEED * time.delta_seconds());
    }

    let axis_angle = transform.rotation.to_axis_angle();
    let current_rotation = axis_angle.0.z * axis_angle.1;

    player_gun.shoot_timer.tick(time.delta());

    if input.pressed(KeyCode::KeyK)
        && player_gun.shoot_timer.finished()
        && player_gun.ammunition != 0
    {
        velocity.0 += Vec2::from_angle(current_rotation) * BOOST_ACCELERATION_SPEED;
        player_gun.ammunition -= 1;
        player_gun.shoot_timer.reset();
        spawn_bullet(transform.clone(), current_rotation, velocity.0, commands);
    }

    let velocity_speed = velocity.0.length();
    velocity.0 += Vec2::from_angle(current_rotation) * PASSIVE_ACCELERATION_SPEED;
    velocity.0 = velocity.0.normalize()
        * if velocity_speed > MAX_SPEED {
            MAX_SPEED
        } else {
            velocity_speed
        };
    velocity.0 *= DRAG;
}

#[derive(Component)]
pub struct Bullet;

fn spawn_bullet(
    player_transform: Transform,
    player_rotation: f32,
    player_velocity: Vec2,
    mut commands: Commands,
) {
    let velocity = (Vec2::from_angle(player_rotation) * BULLET_SPEED * -1.0)
        + Vec2::from_angle(thread_rng().gen_range(-TAU..TAU)) * BULLET_VELOCITY_OFFSET;

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3 {
                    z: 0.0,
                    ..player_transform.translation
                },
                rotation: Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), velocity.to_angle()),
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Option::Some(Vec2::new(8.0, 1.0)),
                ..default()
            },
            ..default()
        },
        Velocity((velocity + player_velocity).normalize() * velocity.length()),
        Bullet,
    ));
}

pub fn delete_bullets(
    mut commands: Commands,
    player_query: Query<&Transform, (With<PlayerGun>, Without<Bullet>)>,
    bullets: Query<(Entity, &Transform), With<Bullet>>,
) {
    let player_transform = player_query.single();

    for (bullet, bullet_transform) in bullets.iter() {
        if bullet_transform
            .translation
            .distance(player_transform.translation)
            > 80.0
        {
            commands.entity(bullet).despawn();
        }
    }
}

pub fn move_objects_with_velocity(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for object in query.iter_mut() {
        let (mut transform, velocity) = object;

        transform.translation.x += velocity.0.x * time.delta_seconds();
        transform.translation.y += velocity.0.y * time.delta_seconds();
    }
}
