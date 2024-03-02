use bevy::prelude::*;
use bevy::render::{
    camera::RenderTarget,
    render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
    view::RenderLayers,
};
use bevy::window::WindowResized;

const RESOLUTION: Extent3d = Extent3d {
    width: 256,
    height: 256,
    depth_or_array_layers: 1,
};
// const PIXEL_PERFECT_LAYER: RenderLayers = RenderLayers::layer(0);
const HIGH_RES_LAYER: RenderLayers = RenderLayers::layer(1);

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct InGameCamera;

#[derive(Component)]
struct Canvas;

#[derive(Component)]
struct OuterCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(Msaa::Off)
        .add_systems(Startup, (setup_player, setup_camera))
        .add_systems(Update, fit_canvas)
        .run();
}

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("graphics/player.png"),
            ..default()
        },
        Velocity { x: 0.0, y: 0.0 },
    ));
}

fn setup_camera(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
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

fn fit_canvas(
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
