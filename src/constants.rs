use bevy::render::{render_resource::Extent3d, view::RenderLayers};

pub const HIGH_RES_LAYER: RenderLayers = RenderLayers::layer(1);
pub const RESOLUTION: Extent3d = Extent3d {
    width: 320,
    height: 180,
    depth_or_array_layers: 1,
};

pub const DOT_DISTANCE: u32 = 10;

pub const CAMERA_FOLLOW_SPEED: f32 = 0.95;
pub const CAMERA_LOOKAHEAD_DISTANCE: f32 = 170.0;

pub const BOOST_ACCELERATION_SPEED: f32 = 11.0;
pub const PASSIVE_ACCELERATION_SPEED: f32 = 5.0;
pub const MAX_SPEED: f32 = 145.0;
pub const ROTATION_SPEED: f32 = 7.0;
pub const DRAG: f32 = 0.998;

pub const ENEMY_ACCELLERATION: f32 = 8.0;
pub const ENEMY_MAX_SPEED: f32 = 150.0;
pub const ENEMY_ROTATION_SPEED: f32 = 4.0;

pub const BULLET_SPEED: f32 = 70.0;
pub const BULLET_VELOCITY_OFFSET: f32 = 30.0;

pub const JERRY_CAN_FUEL_COUNT: u32 = 60;
pub const JERRY_CAN_COLLECT_SPEED: f32 = 0.45;
