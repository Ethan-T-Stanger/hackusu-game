mod camera;
mod constants;
mod enemies;
mod jerry_cans;
mod player;

use bevy::prelude::*;
use {
    bevy::window::WindowMode,
    camera::{add_background_dots, fit_canvas, follow_player, move_background_dots, setup_camera},
    enemies::{collide_with_enemies, move_enemies, setup_enemy_spawn_timer, spawn_enemy},
    jerry_cans::rotate_jerry_cans,
    player::{control_player, delete_bullets, move_objects_with_velocity, setup_player},
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
        .add_systems(
            Startup,
            (
                setup_player,
                setup_camera,
                add_background_dots,
                setup_enemy_spawn_timer,
            ),
        )
        .add_systems(
            Update,
            (
                fit_canvas,
                move_background_dots,
                rotate_jerry_cans,
                spawn_enemy,
                (
                    (
                        (control_player, follow_player, move_enemies),
                        move_objects_with_velocity,
                    )
                        .chain(),
                    (delete_bullets, collide_with_enemies),
                )
                    .chain(),
            ),
        )
        .run();
}
