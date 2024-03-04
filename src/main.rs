mod camera;
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
        .add_systems(Update, (rotate, camera::fit_canvas))
        .run();
}

#[derive(Component)]
struct Rotate;

/// Spawns a capsule mesh on the pixel-perfect layer.
fn setup_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::default()),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Rotate,
        camera::PIXEL_PERFECT_LAYERS,
    ));

    // desk
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cylinder::default()),
            material: materials.add(Color::rgb(0.5, 0.4, 0.3)),
            transform: Transform::from_xyz(0.0, -2.0, 0.0).with_scale(Vec3::new(15.0, 0.1, 15.0)),
            ..default()
        },
        camera::PIXEL_PERFECT_LAYERS,
    ));

    // spotlight
    commands.spawn(SpotLightBundle {
        transform: Transform::from_xyz(0.0, 2.0, 0.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        spot_light: SpotLight {
            intensity: 4000.0, // lumens
            color: Color::WHITE,
            shadows_enabled: true,
            inner_angle: PI / 4.0 * 0.85,
            outer_angle: PI / 4.0,
            ..default()
        },
        ..default()
    });

    // light
    // commands.spawn(DirectionalLightBundle {
    //     directional_light: DirectionalLight {
    //         illuminance: 1_000.,
    //         ..default()
    //     },
    //     ..default()
    // });
}

/// Rotates entities to demonstrate grid snapping.
fn rotate(time: Res<Time>, mut transforms: Query<&mut Transform, With<Rotate>>) {
    for mut transform in &mut transforms {
        let dt = time.delta_seconds();
        transform.rotate_z(dt);
        transform.rotate_x(dt * 0.5);
        transform.rotate_y(dt * 0.3);
    }
}
