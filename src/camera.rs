use std::f32::consts::PI;

use crate::{input::InputState, post_process};

use bevy::{
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
    window::WindowResized,
};

/// In-game resolution width.
const RES_WIDTH: u32 = 320;

/// In-game resolution height.
const RES_HEIGHT: u32 = 180;

/// Default render layers for pixel-perfect rendering.
/// You can skip adding this component, as this is the default.
pub const PIXEL_PERFECT_LAYERS: RenderLayers = RenderLayers::layer(0);

/// Render layers for high-resolution rendering.
const HIGH_RES_LAYERS: RenderLayers = RenderLayers::layer(1);

const ZOOM_SPEED: f32 = 50.0;

/// Low-resolution texture that contains the pixel-perfect world.
/// Canvas itself is rendered to the high-resolution world.
#[derive(Component)]
struct Canvas;

/// Camera that renders the pixel-perfect world to the [`Canvas`].
#[derive(Component)]
pub struct InGameCamera {
    pub angle_x: f32,
    pub angle_y: f32,
    pub zoom: f32,
}

/// Camera that renders the [`Canvas`] (and other graphics on [`HIGH_RES_LAYERS`]) to the screen.
#[derive(Component)]
pub struct OuterCamera;

pub fn setup_camera(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let canvas_size = Extent3d {
        width: RES_WIDTH,
        height: RES_HEIGHT,
        ..default()
    };

    // this Image serves as a canvas representing the low-resolution game screen
    let mut canvas = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: canvas_size,
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

    // fill image.data with zeroes
    canvas.resize(canvas_size);

    let image_handle = images.add(canvas);

    // this camera renders whatever is on `PIXEL_PERFECT_LAYERS` to the canvas
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 15.0, 25.0))
                .looking_at(Vec3::default(), Vec3::Y),
            camera: Camera {
                order: -1,
                target: RenderTarget::Image(image_handle.clone()),
                clear_color: Color::BLACK.into(),
                ..default()
            },
            ..default()
        },
        // Add the setting to the camera.
        // This component is also used to determine on which camera to run the post processing effect.
        post_process::PostProcessSettings {
            intensity: 0.00,
            ..default()
        },
        InGameCamera {
            angle_x: 0.0,
            angle_y: 0.0,
            zoom: 25.0,
        },
        PIXEL_PERFECT_LAYERS,
    ));

    // spawn the canvas
    commands.spawn((
        SpriteBundle {
            texture: image_handle,
            ..default()
        },
        Canvas,
        HIGH_RES_LAYERS,
    ));

    // the "outer" camera renders whatever is on `HIGH_RES_LAYERS` to the screen.
    // here, the canvas and one of the sample sprites will be rendered by this camera
    commands.spawn((Camera2dBundle::default(), OuterCamera, HIGH_RES_LAYERS));
}

/// Scales camera projection to fit the window (integer multiples only).
pub fn fit_canvas(
    mut resize_events: EventReader<WindowResized>,
    mut projections: Query<&mut OrthographicProjection, With<OuterCamera>>,
) {
    for event in resize_events.read() {
        let h_scale = event.width / RES_WIDTH as f32;
        let v_scale = event.height / RES_HEIGHT as f32;
        let mut projection = projections.single_mut();
        projection.scale = 1. / h_scale.min(v_scale).round();
    }
}

pub fn orbit_camera(
    time: Res<Time>,
    input_state: Res<InputState>,
    mut camera_q: Query<(&mut Transform, &mut InGameCamera)>,
) {
    for (mut transform, mut in_game_camera) in &mut camera_q {
        in_game_camera.angle_x -= input_state.delta_x * time.delta_seconds();

        in_game_camera.angle_y -= input_state.delta_y * time.delta_seconds();
        if in_game_camera.angle_y > 0.0 {
            in_game_camera.angle_y = 0.0;
        }
        if in_game_camera.angle_y < -PI {
            in_game_camera.angle_y = -PI;
        }

        in_game_camera.zoom -= input_state.delta_zoom * time.delta_seconds() * ZOOM_SPEED;
        if in_game_camera.zoom < 5.0 {
            in_game_camera.zoom = 5.0;
        }
        if in_game_camera.zoom > 25.0 {
            in_game_camera.zoom = 25.0;
        }
        println!("{0}", in_game_camera.zoom);

        *transform = Transform::from_translation(Vec3::new(0.0, 0.0, in_game_camera.zoom));
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_axis_angle(Vec3::X, in_game_camera.angle_y),
        );
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_axis_angle(Vec3::Y, in_game_camera.angle_x),
        );
    }
}
