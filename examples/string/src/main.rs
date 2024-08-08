use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
};

use bevy_prototype_lyon::prelude::*;
use physics_engine_bevy::{Boxes, PhysicsDebugPlugin, PhysicsPlugin};
fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_plugins(ShapePlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(PhysicsDebugPlugin)
        .add_systems(Startup, setup)
        .add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextStyle {
                    font_size: 50.0,
                    color: Color::srgb(0.0, 1.0, 0.0),
                    font: default(),
                },
            },
        })
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube_mesh = meshes.add(Cuboid::from_length(0.4));
    let cube_material = materials.add(Color::srgb(0.8, 0.7, 0.6));

    let static_cube = commands
        .spawn((
            PbrBundle {
                transform: Transform::from_xyz(-0.2, 2.0, 0.0),
                ..default()
            },
            physics_engine_bevy::RigidBody::new(Boxes::cube(0.05), 1.0)
                .unwrap()
                .with_position(physics_engine_bevy::math::Vec3::new(-0.2, 2.0, 0.0))
                .with_type(physics_engine_bevy::RigidBodyType::Static),
        ))
        .id();
    let mut dynamic_cube = commands
        .spawn((
            PbrBundle {
                mesh: cube_mesh.clone(),
                material: cube_material.clone(),
                transform: Transform::from_xyz(0.0, 2.0, 0.2),
                ..default()
            },
            physics_engine_bevy::RigidBody::new(Boxes::cube(0.2), 1.0)
                .unwrap()
                .with_position(physics_engine_bevy::math::Vec3::new(0.0, 2.0, 0.2)),
        ))
        .id();
    commands.spawn(
        physics_engine_bevy::DistanceJoint::new(static_cube, dynamic_cube)
            .with_length(0.2)
            .with_local_pos2(physics_engine_bevy::math::Vec3::new(0.2, 0.2, 0.2)),
    );
    for i in 0..3 {
        let x = 0.01 * (i + 1) as f32;
        let y = 2.0 - 0.4 * (i + 1) as f32;
        let next_dynamic_cube = commands
            .spawn((
                PbrBundle {
                    mesh: cube_mesh.clone(),
                    material: cube_material.clone(),
                    transform: Transform::from_xyz(x, y, 0.2),
                    ..default()
                },
                physics_engine_bevy::RigidBody::new(Boxes::cube(0.2), 1.0)
                    .unwrap()
                    .with_position(physics_engine_bevy::math::Vec3::new(x, y, 0.2)),
            ))
            .id();
        commands.spawn(
            physics_engine_bevy::DistanceJoint::new(dynamic_cube, next_dynamic_cube)
                .with_length(0.4)
                .with_local_pos1(physics_engine_bevy::math::Vec3::new(-0.2, -0.2, -0.2))
                .with_local_pos2(physics_engine_bevy::math::Vec3::new(0.2, 0.2, 0.2)),
        );
        dynamic_cube = next_dynamic_cube;
    }
    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 2_000_000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
