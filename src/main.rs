mod camera;
mod constants;
mod enemies;
mod jerry_cans;
mod player;

use bevy::prelude::*;
use {
    bevy::window::WindowMode,
    camera::{add_background_dots, fit_canvas, follow_player, move_background_dots, setup_camera},
    enemies::{
        collide_with_enemies, move_enemies, setup_enemy_spawn_timer, spawn_enemy, Enemy,
        EnemySpawnTimer,
    },
    jerry_cans::{
        display_ui_jerry_cans, pickup_jerry_cans, rotate_jerry_cans, JerryCan, UIJerryCan,
    },
    player::{
        control_player, delete_bullets, kill_player, move_objects_with_velocity, setup_player,
        Bullet, PlayerGun,
    },
};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            // .set(WindowPlugin {
            //     primary_window: Some(Window {
            //         mode: WindowMode::BorderlessFullscreen,
            //         ..default()
            //     }),
            //     ..default()
            // }),
        )
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, (setup_camera, add_background_dots))
        .add_systems(
            Update,
            (
                (
                    fit_canvas,
                    move_background_dots,
                    rotate_jerry_cans,
                    spawn_enemy,
                    (
                        (
                            (
                                control_player,
                                (follow_player, display_ui_jerry_cans).chain(),
                                move_enemies,
                            ),
                            move_objects_with_velocity,
                        )
                            .chain(),
                        (
                            delete_bullets,
                            collide_with_enemies,
                            pickup_jerry_cans,
                            kill_player,
                        ),
                    )
                        .chain(),
                ),
                reset_game,
            )
                .chain(),
        )
        .run();
}

fn reset_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>,
    player_query: Query<Entity, With<PlayerGun>>,
    enemies: Query<Entity, With<Enemy>>,
    enemy_spawn_timer_query: Query<Entity, With<EnemySpawnTimer>>,
    bullets: Query<Entity, With<Bullet>>,
    jerry_cans: Query<Entity, With<JerryCan>>,
    ui_jerry_cans: Query<Entity, With<UIJerryCan>>,
) {
    if !input.pressed(KeyCode::KeyR) {
        return;
    }

    match player_query.get_single() {
        Ok(player) => commands.entity(player).despawn(),
        Err(_) => (),
    };

    for enemy in enemies.iter() {
        commands.entity(enemy).despawn();
    }

    match enemy_spawn_timer_query.get_single() {
        Ok(player) => commands.entity(player).despawn(),
        Err(_) => (),
    };

    for bullet in bullets.iter() {
        commands.entity(bullet).despawn();
    }

    for jerry_can in jerry_cans.iter() {
        commands.entity(jerry_can).despawn();
    }

    for ui_jerry_can in ui_jerry_cans.iter() {
        commands.entity(ui_jerry_can).despawn();
    }

    setup_player(&mut commands, &asset_server);
    setup_enemy_spawn_timer(&mut commands)
}
