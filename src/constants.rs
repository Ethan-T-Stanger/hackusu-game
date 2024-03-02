use bevy::render::{render_resource::Extent3d, view::RenderLayers};

pub const HIGH_RES_LAYER: RenderLayers = RenderLayers::layer(1);
pub const RESOLUTION: Extent3d = Extent3d {
    width: 512,
    height: 288,
    depth_or_array_layers: 1,
};

pub const CAMERA_FOLLOW_SPEED: f32 = 0.8;
pub const CAMERA_LOOKAHEAD_DISTANCE: f32 = 80.0;

pub const BOOST_ACCELERATION_SPEED: f32 = 11.0;
pub const PASSIVE_ACCELERATION_SPEED: f32 = 5.0;
pub const MAX_SPEED: f32 = 145.0;
pub const ROTATION_SPEED: f32 = 7.0;
pub const DRAG: f32 = 0.998;

pub const BULLET_SPEED: f32 = 250.0;
pub const BULLET_VELOCITY_OFFSET: f32 = 50.0;
