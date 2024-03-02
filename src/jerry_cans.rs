use bevy::prelude::*;

use {
    crate::{
        camera::InGameCamera,
        constants::{JERRY_CAN_COLLECT_SPEED, JERRY_CAN_FUEL_COUNT, RESOLUTION},
        player::PlayerGun,
    },
    rand::{thread_rng, Rng},
    std::time::Duration,
};

#[derive(Bundle)]
pub struct JerryCanBundle {
    sprite_sheet_bundle: SpriteSheetBundle,
    jerry_can: JerryCan,
}

#[derive(Component)]
pub struct JerryCan {
    pickup_timer: Timer,
    sprite_update_timer: Timer,
}

pub fn spawn_jerry_can(
    position: Vec3,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(JerryCanBundle {
        sprite_sheet_bundle: SpriteSheetBundle {
            texture: asset_server.load("graphics/jerry_can.png"),
            atlas: TextureAtlas {
                layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                    Vec2::new(6.0, 6.0),
                    9,
                    1,
                    None,
                    None,
                )),
                index: 0,
                ..default()
            },
            transform: Transform::from_translation(position),
            ..default()
        },
        jerry_can: JerryCan {
            pickup_timer: Timer::from_seconds(1.0, TimerMode::Once),
            sprite_update_timer: Timer::new(Duration::from_millis(200), TimerMode::Repeating),
        },
    });
}

pub fn rotate_jerry_cans(
    time: Res<Time>,
    mut jerry_cans: Query<(&mut TextureAtlas, &mut JerryCan), Without<UIJerryCan>>,
    mut ui_jerry_cans: Query<(&mut TextureAtlas, &mut UIJerryCan)>,
) {
    for (mut atlas, mut jerry_can) in jerry_cans.iter_mut() {
        jerry_can.sprite_update_timer.tick(time.delta());

        if !jerry_can.sprite_update_timer.just_finished() {
            continue;
        }

        atlas.index = if atlas.index == 8 { 0 } else { atlas.index + 1 };
    }

    for (mut atlas, mut jerry_can) in ui_jerry_cans.iter_mut() {
        jerry_can.0.tick(time.delta());

        if !jerry_can.0.just_finished() {
            continue;
        }

        atlas.index = if atlas.index == 8 { 0 } else { atlas.index + 1 };
    }
}

pub fn pickup_jerry_cans(
    time: Res<Time>,
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut PlayerGun), Without<JerryCan>>,
    mut jerry_cans: Query<(Entity, &mut Transform, &mut JerryCan)>,
) {
    let (player_transform, mut player_gun) = match player_query.get_single_mut() {
        Ok(value) => value,
        Err(_) => return,
    };

    for (jerry_can_entity, mut jerry_can_transform, mut jerry_can) in jerry_cans.iter_mut() {
        jerry_can.pickup_timer.tick(time.delta());

        if jerry_can.pickup_timer.finished() {
            jerry_can_transform.translation = jerry_can_transform
                .translation
                .lerp(player_transform.translation, JERRY_CAN_COLLECT_SPEED);
        }

        if jerry_can_transform
            .translation
            .distance(player_transform.translation)
            < 4.0
        {
            commands.entity(jerry_can_entity).despawn();
            player_gun.ammunition += JERRY_CAN_FUEL_COUNT
        }
    }
}

#[derive(Component)]
pub struct UIJerryCan(Timer);

pub fn display_ui_jerry_cans(
    mut commands: Commands,
    player_query: Query<&PlayerGun>,
    mut ui_jerry_cans: Query<(Entity, &mut Transform), (With<UIJerryCan>, Without<InGameCamera>)>,
    camera_query: Query<&Transform, With<InGameCamera>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let player_gun = match player_query.get_single() {
        Ok(value) => value,
        Err(_) => return,
    };
    let camera = camera_query.single();

    let max_count = (player_gun.ammunition as f32 / JERRY_CAN_FUEL_COUNT as f32).ceil() as i32;
    let mut count = 0;

    for (entity, mut jerry_can) in ui_jerry_cans.iter_mut() {
        if count >= max_count {
            commands.entity(entity).despawn();
            continue;
        }

        jerry_can.translation = Vec3 {
            x: (RESOLUTION.width as f32 / -2.0)
                + 10.0
                + (count as f32 * 9.0)
                + camera.translation.x,
            y: (RESOLUTION.height as f32 / -2.0) + 10.0 + camera.translation.y,
            z: jerry_can.translation.z,
        };

        count += 1;
    }

    if count < max_count {
        for i in count..max_count {
            commands.spawn((
                SpriteSheetBundle {
                    texture: asset_server.load("graphics/jerry_can.png"),
                    atlas: TextureAtlas {
                        layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                            Vec2::new(6.0, 6.0),
                            9,
                            1,
                            None,
                            None,
                        )),
                        index: 0,
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        (RESOLUTION.width as f32 / -2.0) + (i as f32 * 9.0) + 10.0,
                        (RESOLUTION.height as f32 / -2.0) + 10.0,
                        20.0,
                    ),
                    ..default()
                },
                UIJerryCan(Timer::new(
                    Duration::from_millis(thread_rng().gen_range(170..230)),
                    TimerMode::Repeating,
                )),
            ));
        }
    }
}
