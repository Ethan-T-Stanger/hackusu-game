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
    width: 512,
    height: 288,
    depth_or_array_layers: 1,
};

const HIGH_RES_LAYER: RenderLayers = RenderLayers::layer(1);

const BOOST_ACCELERATION_SPEED: f32 = 4.0;
const PASSIVE_ACCELERATION_SPEED: f32 = 3.0;
const MAX_SPEED: f32 = 145.0;
const ROTATION_SPEED: f32 = 7.0;
const DRAG: f32 = 0.998;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(Vec2);

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
        .add_systems(Update, (fit_canvas, move_player))
        .run();
}

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("graphics/player.png"),
            ..default()
        },
        Velocity(Vec2::ZERO),
        Player,
    ));
}

fn move_player(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Player, &mut Transform, &mut Velocity)>,
) {
    let (_player, mut transform, mut velocity) = query.single_mut();

    if input.pressed(KeyCode::KeyA) {
        transform.rotate_z(ROTATION_SPEED * time.delta_seconds());
    }
    if input.pressed(KeyCode::KeyD) {
        transform.rotate_z(-ROTATION_SPEED * time.delta_seconds());
    }

    let axis_angle = transform.rotation.to_axis_angle();
    let current_rotation = axis_angle.0.z * axis_angle.1;

    if input.pressed(KeyCode::KeyK) {
        let acceleration_vector = rotation_to_vector(current_rotation) * BOOST_ACCELERATION_SPEED;
        velocity.0 += acceleration_vector;
    }

    let velocity_speed = velocity.0.length();
    velocity.0 += rotation_to_vector(current_rotation) * PASSIVE_ACCELERATION_SPEED;
    velocity.0 = velocity.0.normalize()
        * if velocity_speed > MAX_SPEED {
            MAX_SPEED
        } else {
            velocity_speed
        };
    velocity.0 *= DRAG;

    transform.translation.x += velocity.0.x * time.delta_seconds();
    transform.translation.y += velocity.0.y * time.delta_seconds();
}

fn rotation_to_vector(rotation: f32) -> Vec2 {
    Vec2 {
        x: rotation.cos(),
        y: rotation.sin(),
    }
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
