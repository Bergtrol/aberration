//! Shows how to create graphics that snap to the pixel grid by rendering to a texture in 2D

mod camera;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
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
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Capsule2d::default()).into(),
            transform: Transform::from_xyz(40., 0., 2.).with_scale(Vec3::splat(32.)),
            material: materials.add(Color::BLACK),
            ..default()
        },
        Rotate,
        camera::PIXEL_PERFECT_LAYERS,
    ));
}

/// Rotates entities to demonstrate grid snapping.
fn rotate(time: Res<Time>, mut transforms: Query<&mut Transform, With<Rotate>>) {
    for mut transform in &mut transforms {
        let dt = time.delta_seconds();
        transform.rotate_z(dt);
    }
}
