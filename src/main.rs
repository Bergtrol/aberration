mod camera;
mod input;
mod post_process;

use std::f32::consts::PI;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            post_process::PostProcessPlugin,
        ))
        .insert_resource(Msaa::Off)
        .add_systems(Startup, (camera::setup_camera, setup_mesh))
        .insert_resource(input::InputState {
            moving_camera: false,
        })
        .add_systems(
            Update,
            (camera::fit_canvas, input::mouse_motion, input::grab_mouse),
        )
        .run();
}

/// Spawns a capsule mesh on the pixel-perfect layer.
fn setup_mesh(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn the first scene
    commands.spawn(SceneBundle {
        scene: asset_server.load("dollhouse.gltf#Scene0"),
        ..default()
    });

    // spotlight
    // commands.spawn(SpotLightBundle {
    //     transform: Transform::from_xyz(0.0, 20.0, 0.0)
    //         .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::X),
    //     spot_light: SpotLight {
    //         intensity: 400000.0, // lumens
    //         color: Color::WHITE,
    //         shadows_enabled: true,
    //         inner_angle: PI / 4.0 * 0.85,
    //         outer_angle: PI / 4.0,
    //         ..default()
    //     },
    //     ..default()
    // });

    // Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            PI * -0.15,
            PI * -0.15,
        ))
        .into(),
        ..default()
    });
}
