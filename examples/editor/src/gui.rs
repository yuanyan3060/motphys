use crate::{
    common::{GameState, Picked, TransformStore},
    toggle_switch::toggle_ui,
    JointSetting, LinkTarget,
};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use physics_engine_bevy::{
    math::Quaternion, utils::bevy_vec3_2_engine, Boxes, DistanceJoint, RigidBody, RigidBodyType,
};

#[derive(Component, PartialEq, Clone)]
pub struct Cube {
    pub size: Vec3,
    pub mass: f32,
    pub position: Vec3,
    pub rotation: Vec3,
    pub is_static: bool,
}

impl Default for Cube {
    fn default() -> Self {
        Self {
            size: Vec3::ONE,
            mass: 1.0,
            position: Default::default(),
            rotation: Default::default(),
            is_static: false,
        }
    }
}
#[allow(clippy::type_complexity)]
pub fn ui_loop(
    commands: Commands,
    mut contexts: EguiContexts,
    mut param_set: ParamSet<(
        Query<(Entity, &mut Transform, &mut Cube)>,
        Query<(Entity, &mut Transform, &mut LinkTarget)>,
    )>,
    joint_query: Query<(Entity, &DistanceJoint<Entity>)>,
    transform_store: ResMut<TransformStore>,
    picked: Res<Picked>,
    state: ResMut<GameState>,
) {
    egui::Window::new("Tools")
        .fixed_pos((0.0, 100.0))
        .show(contexts.ctx_mut(), |ui| {
            let is_play = state.is_play();
            on_none(
                commands,
                ui,
                &mut param_set,
                joint_query,
                transform_store,
                state,
            );
            if is_play {
                return;
            }
            match *picked {
                Picked::None => {}
                Picked::Cuboid(entity) => {
                    let mut query = param_set.p0();
                    let Ok((_, mut transform, mut cube)) = query.get_mut(entity) else {
                        return;
                    };
                    let pre_cube = cube.clone();
                    on_cube(ui, &mut cube);
                    if *cube == pre_cube {
                        return;
                    }
                    transform.translation = cube.position;
                    transform.rotation = Quat::from_euler(
                        EulerRot::YXZ,
                        cube.rotation.x.to_radians(),
                        cube.rotation.y.to_radians(),
                        cube.rotation.z.to_radians(),
                    );
                    transform.scale = cube.size;
                    let size = cube.size;
                    //*mesh = meshes.add(Cuboid::from_size(cube.size));
                    //let delta = (cube.size - size) / 2.0;
                    for (_, mut link_transform, link) in param_set.p1().iter_mut() {
                        if link.cube != entity {
                            continue;
                        }
                        link_transform.scale = 1.0 / size;
                    }
                }
                Picked::Joint(entity) => {
                    let mut query = param_set.p1();
                    let Ok((_, _, mut link)) = query.get_mut(entity) else {
                        return;
                    };
                    if let Some(x) = link.setting.as_mut() {
                        on_joint(ui, x)
                    }
                }
            }
        });
}

#[allow(clippy::type_complexity)] 
fn on_none(
    mut commands: Commands,
    ui: &mut egui::Ui,
    param_set: &mut ParamSet<(
        Query<(Entity, &mut Transform, &mut Cube)>,
        Query<(Entity, &mut Transform, &mut LinkTarget)>,
    )>,
    joint_query: Query<(Entity, &DistanceJoint<Entity>)>,
    mut transform_store: ResMut<TransformStore>,
    mut state: ResMut<GameState>,
) {
    ui.horizontal(|ui| {
        if ui.button("play").clicked() {
            *state = GameState::Play;
            param_set.p0().iter().for_each(|(entity, transform, cube)| {
                let shape = Boxes::new(cube.size.x / 2.0, cube.size.y / 2.0, cube.size.z / 2.0);
                let body = RigidBody::new(shape, cube.mass)
                    .expect("invalid shape or mass")
                    .with_position(physics_engine_bevy::math::Vec3::new(
                        transform.translation.x,
                        transform.translation.y,
                        transform.translation.z,
                    ))
                    .with_quaternion(Quaternion::new(
                        transform.rotation.w,
                        transform.rotation.x,
                        transform.rotation.y,
                        transform.rotation.z,
                    ))
                    .with_type(if cube.is_static {
                        RigidBodyType::Static
                    } else {
                        RigidBodyType::Dynamic
                    });
                commands.entity(entity).insert(body);
                transform_store.inner.insert(entity, *transform);
            });
            let qurey = param_set.p1();
            qurey
                .iter()
                .for_each(|(cube1_entity, cube1_transform, link1)| {
                    let (Some(setting), Some(target)) = (&link1.setting, link1.target_link) else {
                        return;
                    };
                    let Ok((cube2_entity, cube2_transform, link2)) = qurey.get(target) else {
                        return;
                    };
                    commands.spawn(
                        DistanceJoint::new(link1.cube, link2.cube)
                            .with_length(setting.distance)
                            .with_local_pos1(bevy_vec3_2_engine(
                                cube1_transform.translation / cube1_transform.scale,
                            ))
                            .with_local_pos2(bevy_vec3_2_engine(
                                cube2_transform.translation / cube2_transform.scale,
                            )),
                    );
                    transform_store.inner.insert(cube1_entity, *cube1_transform);
                    transform_store.inner.insert(cube2_entity, *cube2_transform);
                });
        };
        if ui.button("restart").clicked() {
            *state = GameState::Editor;
            param_set
                .p0()
                .iter_mut()
                .for_each(|(entity, mut transform, _)| {
                    if let Some(x) = transform_store.inner.get(&entity) {
                        *transform = *x;
                    };
                    commands.entity(entity).remove::<RigidBody>();
                });
            param_set
                .p1()
                .iter_mut()
                .for_each(|(entity, mut transform, _)| {
                    if let Some(store) = transform_store.inner.get(&entity) {
                        *transform = *store;
                    };
                });
            joint_query.iter().for_each(|(x, _)| {
                commands.entity(x).despawn();
            });
            transform_store.inner.clear()
        };
        if ui.button("clear").clicked() {
            *state = GameState::Editor;
            param_set
                .p0()
                .iter()
                .for_each(|(entity, _, _)| commands.entity(entity).despawn());
            param_set
                .p1()
                .iter()
                .for_each(|(entity, _, _)| commands.entity(entity).despawn());
            transform_store.inner.clear()
        };
    });
}

fn on_cube(ui: &mut egui::Ui, cube: &mut Cube) {
    ui.horizontal(|ui| {
        // 保持标题不变

        // 使用 vertical 方法将控件竖直排列
        ui.vertical(|ui| {
            ui.label("Cube Properties");
            // 控制长方体的大小
            ui.label("Size:");

            // 对齐每行控件
            ui.horizontal(|ui| {
                ui.add(egui::Slider::new(&mut cube.size.x, 0.1..=5.0).text("Width"));
            });
            ui.horizontal(|ui| {
                ui.add(egui::Slider::new(&mut cube.size.y, 0.1..=5.0).text("Height"));
            });
            ui.horizontal(|ui| {
                ui.add(egui::Slider::new(&mut cube.size.z, 0.1..=5.0).text("Depth"));
            });

            // 控制质量
            ui.label("Mass:");
            ui.horizontal(|ui| {
                ui.add(egui::Slider::new(&mut cube.mass, 0.1..=10.0).text("Mass"));
            });

            // 控制位置
            ui.label("Position:");
            ui.horizontal(|ui| {
                ui.add(egui::Slider::new(&mut cube.position.x, -10.0..=10.0).text("X"));
            });
            ui.horizontal(|ui| {
                ui.add(egui::Slider::new(&mut cube.position.y, -10.0..=10.0).text("Y"));
            });
            ui.horizontal(|ui| {
                ui.add(egui::Slider::new(&mut cube.position.z, -10.0..=10.0).text("Z"));
            });

            // 控制姿态（旋转）
            ui.label("Rotation (Degrees):");
            ui.horizontal(|ui| {
                ui.add(egui::Slider::new(&mut cube.rotation.x, 0.0..=360.0).text("a"));
            });
            ui.horizontal(|ui| {
                ui.add(egui::Slider::new(&mut cube.rotation.y, 0.0..=360.0).text("b"));
            });
            ui.horizontal(|ui| {
                ui.add(egui::Slider::new(&mut cube.rotation.z, 0.0..=360.0).text("c"));
            });
            ui.horizontal(|ui| {
                ui.label("Static");
                toggle_ui(ui, &mut cube.is_static);
            })
        });
    });
}

fn on_joint(ui: &mut egui::Ui, joint: &mut JointSetting) {
    ui.horizontal(|ui| {
        // 保持标题不变

        // 使用 vertical 方法将控件竖直排列
        ui.vertical(|ui| {
            ui.label("Joint Properties");

            // 对齐每行控件
            ui.horizontal(|ui| {
                ui.add(egui::Slider::new(&mut joint.distance, 0.1..=10.0).text("Distance"));
            });
        });
    });
}
