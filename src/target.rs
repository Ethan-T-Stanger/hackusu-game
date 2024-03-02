use bevy::prelude::*;

use crate::constants::JERRY_CAN_FUEL_COUNT;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use rand::{thread_rng, Rng};

use {
    crate::{camera::InGameCamera, constants::RESOLUTION, player::PlayerStats},
    std::time::Duration,
};

#[derive(Component)]
pub struct Arrow;

#[derive(Component)]
pub struct Target;

pub fn display_arrow(
    mut commands: Commands,
    mut arrow_query: Query<&mut Transform, (With<Arrow>, Without<PlayerStats>, Without<Target>)>,
    player_query: Query<&Transform, (With<PlayerStats>, Without<Target>)>,
    target_query: Query<&Transform, With<Target>>,
    asset_server: Res<AssetServer>,
) {
    fn get_arrow_position(player_transform: &Transform, target_transform: &Transform) -> Transform {
        let angle_to_target = (Vec2::new(
            target_transform.translation.x,
            target_transform.translation.y,
        ) - Vec2::new(
            player_transform.translation.x,
            player_transform.translation.y,
        ))
        .to_angle();

        return Transform {
            translation: (target_transform.translation - player_transform.translation).normalize()
                * 20.0
                + player_transform.translation,
            rotation: Quat::from_axis_angle(
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: angle_to_target.abs() / angle_to_target,
                },
                angle_to_target.abs(),
            ),
            ..default()
        };
    }

    let player_transform = match player_query.get_single() {
        Ok(value) => value,
        Err(_) => return,
    };

    let target_transform = match target_query.get_single() {
        Ok(value) => value,
        Err(_) => return,
    };

    let mut arrow_transform = match arrow_query.get_single_mut() {
        Ok(value) => value,
        Err(_) => {
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("graphics/arrow.png"),
                    transform: get_arrow_position(player_transform, target_transform),
                    ..default()
                },
                Arrow,
            ));
            return;
        }
    };

    let temp_transform = get_arrow_position(player_transform, target_transform);
    arrow_transform.translation = temp_transform.translation;
    arrow_transform.rotation = temp_transform.rotation;
}

pub fn setup_target(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let rand_location =
        Vec2::from_angle(thread_rng().gen_range(1.0..2.5)) * thread_rng().gen_range(50.0..450.0);

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 24.0 })),
            material: materials.add(Color::rgba(1.0, 0.7, 0.1, 0.5)),
            transform: Transform {
                translation: Vec3 {
                    x: rand_location.x,
                    y: rand_location.y,
                    z: 0.0,
                },
                ..default()
            },
            ..default()
        },
        Target,
    ));
}

pub fn touch_target(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut player_query: Query<(&Transform, &mut PlayerStats)>,
    target_query: Query<(Entity, &Transform), With<Target>>,
    asset_server: Res<AssetServer>,
) {
    let (player_transform, mut player_stats) = match player_query.get_single_mut() {
        Ok(value) => value,
        Err(_) => return,
    };

    let (target_entity, target_transform) = match target_query.get_single() {
        Ok(value) => value,
        Err(_) => return,
    };

    if player_transform.translation.distance(Vec3 {
        z: player_transform.translation.z,
        ..target_transform.translation
    }) < 24.0
    {
        commands.entity(target_entity).despawn();
        setup_target(&mut commands, &mut meshes, &mut materials);
        if player_stats.score < 35 {
            player_stats.score += 1;
            player_stats.ammunition += JERRY_CAN_FUEL_COUNT;
        }

        commands.spawn(AudioBundle {
            source: asset_server.load("sfx/checkpoint.ogg"),
            ..default()
        });
    }
}

#[derive(Component)]
pub struct Star(pub Timer);

pub fn display_stars(
    mut commands: Commands,
    player_query: Query<&PlayerStats>,
    mut stars: Query<&mut Transform, (With<Star>, Without<InGameCamera>)>,
    camera_query: Query<&Transform, With<InGameCamera>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let player_gun = match player_query.get_single() {
        Ok(value) => value,
        Err(_) => return,
    };
    let camera = camera_query.single();

    let max_count = player_gun.score;
    let mut count = 0;

    for mut star in stars.iter_mut() {
        star.translation = Vec3 {
            x: (RESOLUTION.width as f32 / -2.0) + (count as f32 * 9.0) + camera.translation.x + 8.0,
            y: (RESOLUTION.height as f32 / 2.0) - 10.0 + camera.translation.y,
            z: star.translation.z,
        };

        count += 1;
    }

    if count < max_count {
        for i in count..max_count {
            commands.spawn((
                SpriteSheetBundle {
                    texture: asset_server.load("graphics/star.png"),
                    atlas: TextureAtlas {
                        layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                            Vec2::new(8.0, 8.0),
                            8,
                            1,
                            None,
                            None,
                        )),
                        index: 0,
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        (RESOLUTION.width as f32 / -2.0) + (i as f32 * 9.0) + 8.0,
                        (RESOLUTION.height as f32 / 2.0) - 10.0,
                        20.0,
                    ),
                    ..default()
                },
                Star(Timer::new(
                    Duration::from_millis(thread_rng().gen_range(170..230)),
                    TimerMode::Repeating,
                )),
            ));
        }
    }
}
