pub mod utils;
use bevy::app::{FixedUpdate, Plugin};
use bevy::{color::palettes::css::RED, prelude::*};
pub use physics_engine_core::*;
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(FixedUpdate, physics_engine_update);
    }
}

pub fn physics_engine_update(
    time: Res<Time>,
    mut query: Query<(&mut RigidBody, &mut Transform)>,
    joint_query: Query<&DistanceJoint<Entity>>,
) {
    let dt = time.delta_seconds();
    let mut combinations = query.iter_combinations_mut();
    while let Some([(mut body1, _), (mut body2, _)]) = combinations.fetch_next() {
        obb::resolve_collision(&mut body1, &mut body2);
    }
    query.par_iter_mut().for_each(|(mut body, mut trans)| {
        if body.body_type == RigidBodyType::Static {
            return;
        }
        apply_gravity(&mut body);
        apply_air_resistance(&mut body);
        body.update(dt);
        trans.translation = utils::engine_vec3_2_bevy(body.position);
        trans.rotation = utils::engine_quat_2_bevy(body.quaternion)
    });
    for joint in joint_query.iter() {
        let Ok([mut entity1, mut entity2]) = query.get_many_mut([joint.id1, joint.id2]) else {
            continue;
        };
        joint.apply(&mut entity1.0, &mut entity2.0);
    }
}

pub struct PhysicsDebugPlugin;

impl Plugin for PhysicsDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, draw_distance_lines);
    }
}

pub fn draw_distance_lines(
    mut query: Query<&mut RigidBody>,
    joint_query: Query<&DistanceJoint<Entity>>,
    mut gizmos: Gizmos,
) {
    for joint in joint_query.iter() {
        let Ok([entity1, entity2]) = query.get_many_mut([joint.id1, joint.id2]) else {
            continue;
        };
        let pos1 = entity1.local_position_2_world(joint.local_pos1);
        let pos1 = utils::engine_vec3_2_bevy(pos1);
        let pos2 = entity2.local_position_2_world(joint.local_pos2);
        let pos2 = utils::engine_vec3_2_bevy(pos2);
        gizmos.linestrip([pos1, pos2], RED);
    }
}

fn apply_gravity(body: &mut RigidBody) {
    let gravity = physics_engine_core::math::Vec3::new(0.0, -9.8 * body.mass, 0.0);
    let position = body.position;
    body.apply_force(gravity, position);
}

fn apply_air_resistance(body: &mut RigidBody) {
    let velocity = body.velocity;
    let speed = velocity.length();
    if speed > 1e-6 {
        let drag_coefficient = 0.6;
        let air_density = 1.8;
        let area = 0.2;
        let drag_force_magnitude = 0.5 * air_density * speed * speed * drag_coefficient * area;
        let drag_force = -velocity / speed * drag_force_magnitude;
        // 修正速度
        body.apply_force(drag_force, body.position);
    }
    // 修正角速度
    body.apply_torque(body.angular_velocity * -0.1)
}
