mod camera;
mod constants;
mod game;

use bevy::prelude::*;
use {
    camera::{fit_canvas, setup_camera},
    game::{move_player, setup_player},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(Msaa::Off)
        .add_systems(Startup, (setup_player, setup_camera))
        .add_systems(Update, (fit_canvas, move_player))
        .run();
}
