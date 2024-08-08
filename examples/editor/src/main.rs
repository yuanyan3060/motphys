mod common;
mod gui;
mod toggle_switch;
mod util;

use backend::HitData;
use bevy::{
    color::palettes::css::RED,
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
};
use bevy_egui::{self};
use bevy_mod_picking::prelude::*;
use bevy_prototype_lyon::prelude::*;
use common::{GameState, Picked, TransformStore};
use focus::HoverMap;
use gui::{ui_loop, Cube};
use physics_engine_bevy::{PhysicsDebugPlugin, PhysicsPlugin};
use util::get_pick_entity;
fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_plugins(ShapePlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(PhysicsDebugPlugin)
        .add_plugins(bevy_egui::EguiPlugin)
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
        .add_systems(Update, create_cube)
        .add_systems(Update, link)
        .add_systems(Update, remove)
        .add_systems(Update, ui_loop)
        .add_systems(Update, update_picked)
        .add_systems(Update, draw_distance_lines)
        .insert_resource(Picked::None)
        .insert_resource(GameState::Editor)
        .insert_resource(TransformStore::default())
        .run();
}

fn setup(mut commands: Commands) {
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

fn create_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    windows: Query<&Window>,
    query: Query<(&Camera, &GlobalTransform)>,
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<GameState>,
) {
    if !state.is_editor() {
        return;
    }
    if !keys.just_pressed(KeyCode::KeyC) {
        return;
    }
    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };
    let (camera, camera_transform) = query.single();
    let Some(pos) = screen_to_world(cursor_position, camera, camera_transform) else {
        return;
    };
    let cube_mesh = meshes.add(Cuboid::default());
    let cube_material = materials.add(Color::srgb(0.8, 0.7, 0.6));
    commands.spawn((
        PbrBundle {
            mesh: cube_mesh.clone(),
            material: cube_material.clone(),
            transform: Transform::from_xyz(pos.x, pos.y, pos.z),
            ..default()
        },
        PickableBundle::default(),
        LinkAble,
        //On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
        //    transform.translation.x += drag.delta.x / 200.0;
        //    transform.translation.y -= drag.delta.y / 200.0;
        //}),
        Cube {
            position: pos,
            ..Default::default()
        },
    ));
}

fn screen_to_world(
    cursor_position: Vec2,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Vec3> {
    let ray = camera.viewport_to_world(camera_transform, cursor_position)?;
    Some(ray.get_point(10.0))
}

#[derive(Component, Debug, Clone, Reflect, Default)]
pub struct Linked {
    pos: Vec3,
    target: Option<(Entity, Vec3)>,
}

impl Linked {
    pub fn new(pos: Vec3) -> Self {
        Self { pos, target: None }
    }
}

#[allow(clippy::too_many_arguments)]
fn link(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<GameState>,
    hover_map: Res<HoverMap>,
    query: Query<&GlobalTransform, With<LinkAble>>,
    mut link_query: Query<(Entity, &GlobalTransform, &mut LinkTarget), Without<LinkAble>>,
) {
    if !state.is_editor() {
        return;
    }
    if !keys.just_pressed(KeyCode::KeyL) {
        return;
    }
    let Some((entity, hitdata)) = get_current_hover(&hover_map) else {
        return;
    };
    let Ok(transform) = query.get(*entity) else {
        return;
    };
    let Some(world_pos) = hitdata.position else {
        return;
    };
    let local_pos = transform
        .compute_matrix()
        .inverse()
        .transform_point3(world_pos);
    let ball = match link_query.iter_mut().find(|(_, _, x)| x.target_link.is_none()) {
        Some((other_entity, other_pos, mut other_link)) => {
            let ball = commands
                .spawn((
                    PbrBundle {
                        mesh: meshes.add(Sphere::new(0.05)),
                        material: materials.add(Color::srgb(0.8, 0.0, 0.0)),
                        transform: Transform::from_xyz(local_pos.x, local_pos.y, local_pos.z),
                        ..default()
                    },
                    LinkTarget {
                        target_cube: Some(other_link.cube),
                        target_link: Some(other_entity),
                        cube: *entity,
                        setting: Some(JointSetting {
                            distance: world_pos.distance(other_pos.translation()),
                        }),
                    },
                    PickableBundle::default(),
                ))
                .id();
            other_link.target_link = Some(ball);
            other_link.target_cube = Some(*entity);
            ball
        }
        None => commands
            .spawn((
                PbrBundle {
                    mesh: meshes.add(Sphere::new(0.05)),
                    material: materials.add(Color::srgb(0.8, 0.0, 0.0)),
                    transform: Transform::from_xyz(local_pos.x, local_pos.y, local_pos.z),
                    ..default()
                },
                LinkTarget {
                    target_cube: None,
                    target_link: None,
                    cube: *entity,
                    setting: None,
                },
                PickableBundle::default(),
            ))
            .id(),
    };
    commands.entity(*entity).add_child(ball);
}

fn update_picked(
    mut picked: ResMut<Picked>,
    state: Res<GameState>,
    query1: Query<(Entity, &PickSelection), With<LinkAble>>,
    query2: Query<(Entity, &LinkTarget, &PickSelection)>,
) {
    if !state.is_editor() {
        return;
    }
    for (entity, select) in query1.iter() {
        if !select.is_selected {
            continue;
        }
        *picked = Picked::Cuboid(entity);
        return;
    }
    for (entity, target, select) in query2.iter() {
        if !select.is_selected {
            continue;
        }
        if target.setting.is_some() {
            *picked = Picked::Joint(entity);
            return;
        }
        target.target_link.inspect(|x| {
            *picked = Picked::Joint(*x);
        });
        return;
    }
}

fn remove(
    mut commands: Commands,
    state: Res<GameState>,
    keys: Res<ButtonInput<KeyCode>>,
    query: Query<(Entity, &PickSelection)>,
    cube_query: Query<(Entity, &LinkAble)>,
    link_query: Query<(Entity, &LinkTarget)>,
) {
    if !state.is_editor() {
        return;
    }
    if !keys.just_pressed(KeyCode::KeyR) {
        return;
    }
    let Some(pick) = get_pick_entity(&query) else {
        return;
    };
    commands.entity(pick).despawn();
    if let Ok((_, link)) = link_query.get(pick) {
        // 如果删除的是节点 那么同时删除成对的节点
        if let Some(x) = link.target_link { commands.entity(x).despawn() }
    }
    if let Ok((cube, _)) = cube_query.get(pick) {
        // 如果删除的是方块 那么删除与之相连的所有节点 和成对的节点
        for (entity, link) in link_query.iter() {
            if link.cube == cube || link.target_cube == Some(cube) {
                if let Some(mut x) = commands.get_entity(entity) { x.despawn() }
            }
        }
    }
}

#[derive(Component)]
pub struct LinkAble;

#[derive(Component)]
pub struct LinkTarget {
    pub target_cube: Option<Entity>,
    pub target_link: Option<Entity>,
    pub cube: Entity,
    pub setting: Option<JointSetting>,
}

pub struct JointSetting {
    pub distance: f32,
}

fn get_current_hover(hover_map: &HoverMap) -> Option<(&Entity, &HitData)> {
    hover_map.values().next().and_then(|x| x.iter().next())
}

pub fn draw_distance_lines(
    mut gizmos: Gizmos,
    link_query: Query<(Entity, &LinkTarget)>,
    trans_query: Query<&GlobalTransform>,
) {
    for (entity, link) in link_query.iter() {
        if link.setting.is_some() {
            continue;
        }
        let Some(target) = link.target_link else {
            continue;
        };
        let Ok([entity, target]) = trans_query.get_many([entity, target]) else {
            continue;
        };
        gizmos.linestrip([entity.translation(), target.translation()], RED);
    }
}
