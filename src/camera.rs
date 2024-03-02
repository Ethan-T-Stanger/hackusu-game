use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::render_resource::{
    TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use bevy::window::WindowResized;

use crate::constants::{HIGH_RES_LAYER, RESOLUTION};

#[derive(Component)]
struct InGameCamera;

#[derive(Component)]
pub struct OuterCamera;

#[derive(Component)]
struct Canvas;

pub fn setup_camera(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let mut canvas = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: RESOLUTION,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    canvas.resize(RESOLUTION);

    let image_handle = images.add(canvas);

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: -1,
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            ..default()
        },
        InGameCamera,
    ));

    commands.spawn((
        SpriteBundle {
            texture: image_handle,
            ..default()
        },
        Canvas,
        HIGH_RES_LAYER,
    ));

    commands.spawn((Camera2dBundle::default(), OuterCamera, HIGH_RES_LAYER));
}

pub fn fit_canvas(
    mut resize_events: EventReader<WindowResized>,
    mut projections: Query<&mut OrthographicProjection, With<OuterCamera>>,
) {
    for event in resize_events.read() {
        let h_scale = event.width / RESOLUTION.width as f32;
        let v_scale = event.height / RESOLUTION.height as f32;
        let mut projection = projections.single_mut();
        projection.scale = 1. / h_scale.min(v_scale).round();
    }
}
