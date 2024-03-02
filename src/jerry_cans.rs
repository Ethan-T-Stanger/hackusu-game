use bevy::prelude::*;

use {
    crate::{
        constants::{JERRY_CAN_COLLECT_SPEED, JERRY_CAN_FUEL_COUNT},
        player::PlayerGun,
    },
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
    mut jerry_cans: Query<(&mut TextureAtlas, &mut JerryCan)>,
) {
    for (mut atlas, mut jerry_can) in jerry_cans.iter_mut() {
        jerry_can.sprite_update_timer.tick(time.delta());

        if !jerry_can.sprite_update_timer.just_finished() {
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
    let (player_transform, mut player_gun) = player_query.single_mut();

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
