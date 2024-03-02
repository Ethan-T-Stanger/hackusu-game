use bevy::prelude::*;
use {
    crate::constants::{
        BOOST_ACCELERATION_SPEED, BULLET_SPEED, BULLET_VELOCITY_OFFSET, DRAG, MAX_SPEED,
        PASSIVE_ACCELERATION_SPEED, ROTATION_SPEED,
    },
    bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    rand::{random, thread_rng, Rng},
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
            shoot_timer: Timer::new(Duration::from_millis(5), TimerMode::Once),
            ammunition: 1000,
        },
    });
}

pub fn control_player(
    time: Res<Time>,
    commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut PlayerGun, &mut Transform, &mut Velocity)>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
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
        spawn_bullets(
            10,
            transform.clone(),
            current_rotation,
            commands,
            meshes,
            materials,
        );
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
pub struct Bullet {
    timer: Timer,
}

fn spawn_bullets(
    count: u32,
    player_transform: Transform,
    player_rotation: f32,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let velocity = (Vec2::from_angle(player_rotation) * BULLET_SPEED * -1.0)
        + Vec2::from_angle(thread_rng().gen_range(-TAU..TAU)) * BULLET_VELOCITY_OFFSET;

    for _i in 0..count {
        let color_int = thread_rng().gen_range(0..6);

        let color = if color_int <= 1 {
            Color::rgb(0.75, 0.1, 0.1)
        } else if color_int <= 2 {
            Color::rgb(0.86, 0.38, 0.1)
        } else if color_int <= 4 {
            Color::rgb(0.86, 0.63, 0.1)
        } else {
            Color::rgb(0.2, 0.2, 0.2)
        };

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle {
                    radius: thread_rng().gen_range(1.0..2.5),
                })),
                material: materials.add(color),
                transform: Transform {
                    translation: Vec3 {
                        z: 0.0,
                        ..player_transform.translation
                    },
                    rotation: Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), velocity.to_angle()),
                    ..default()
                },
                ..default()
            },
            Velocity(velocity),
            Bullet {
                timer: Timer::new(
                    Duration::from_millis(thread_rng().gen_range(50..250)),
                    TimerMode::Once,
                ),
            },
        ));
    }
}

pub fn delete_bullets(
    time: Res<Time>,
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Transform, &mut Bullet)>,
) {
    for (bullet, mut transform, mut bullet_timer) in bullets.iter_mut() {
        bullet_timer.timer.tick(time.delta());

        // transform.scale = transform.scale.lerp(
        //     Vec3::ZERO,
        //     bullet_timer.timer.elapsed().as_millis() as f32
        //         / bullet_timer.timer.duration().as_millis() as f32
        //         / 5.0,
        // );

        if bullet_timer.timer.finished() {
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
