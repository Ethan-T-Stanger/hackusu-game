use crate::constants::{
    CAMERA_FOLLOW_SPEED, CAMERA_LOOKAHEAD_DISTANCE, DOT_DISTANCE, HIGH_RES_LAYER, MAX_SPEED,
    RESOLUTION,
};
use crate::game::{PlayerGun, Velocity};
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::render_resource::{
    TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use bevy::window::WindowResized;

#[derive(Component)]
pub struct InGameCamera;

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
                clear_color: ClearColorConfig::Custom(Color::rgb(0.5, 0.5, 0.6)),
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
        projection.scale = 1. / h_scale.min(v_scale).floor();
    }
}

pub fn follow_player(
    time: Res<Time>,
    player_query: Query<(&Transform, &Velocity), (With<PlayerGun>, Without<InGameCamera>)>,
    mut camera_query: Query<&mut Transform, With<InGameCamera>>,
) {
    let (player_transform, velocity) = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    let lookahead_position = (velocity.0 / MAX_SPEED) * CAMERA_LOOKAHEAD_DISTANCE;

    camera_transform.translation = camera_transform.translation.lerp(
        Vec3::new(
            player_transform.translation.x + lookahead_position.x,
            player_transform.translation.y + lookahead_position.y,
            player_transform.translation.z,
        ),
        CAMERA_FOLLOW_SPEED * time.delta_seconds(),
    );
}

#[derive(Component)]
pub struct BackgroundDot;

pub fn add_background_dots(mut commands: Commands) {
    for i in 0..(RESOLUTION.width / DOT_DISTANCE) {
        for j in 0..(RESOLUTION.height / DOT_DISTANCE) {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.4, 0.4, 0.45),
                        custom_size: Option::Some(Vec2::new(1.0, 1.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        i as f32 * DOT_DISTANCE as f32 - RESOLUTION.width as f32 / 2.0 + 2.0,
                        j as f32 * DOT_DISTANCE as f32 - RESOLUTION.height as f32 / 2.0 + 1.0,
                        -1.0,
                    ),
                    ..default()
                },
                BackgroundDot,
            ));
        }
    }
}

pub fn move_background_dots(
    mut dots: Query<&mut Transform, (With<BackgroundDot>, Without<InGameCamera>)>,
    camera_query: Query<&Transform, With<InGameCamera>>,
) {
    let camera = camera_query.single();

    for mut dot in dots.iter_mut() {
        while dot.translation.x - camera.translation.x > RESOLUTION.width as f32 / 2.0 {
            dot.translation.x -= RESOLUTION.width as f32;
        }
        while dot.translation.x - camera.translation.x < RESOLUTION.width as f32 / -2.0 {
            dot.translation.x += RESOLUTION.width as f32;
        }
        while dot.translation.y - camera.translation.y > RESOLUTION.height as f32 / 2.0 {
            dot.translation.y -= RESOLUTION.height as f32;
        }
        while dot.translation.y - camera.translation.y < RESOLUTION.height as f32 / -2.0 {
            dot.translation.y += RESOLUTION.height as f32;
        }
    }
}
