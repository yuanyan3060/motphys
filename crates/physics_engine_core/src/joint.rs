use crate::{math::{Scalar, Vec3}, RigidBody};
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "use_bevy", derive(bevy::prelude::Component))]
pub struct DistanceJoint<Id: Eq> {
    pub id1: Id,
    pub id2: Id,
    pub local_pos1: Vec3,
    pub local_pos2: Vec3,
    pub rest_length: Scalar,
}

impl<Id: Eq> DistanceJoint<Id> {
    pub fn new(id1: Id, id2: Id) -> Self {
        Self {
            id1,
            id2,
            local_pos1: Vec3::ZERO,
            local_pos2: Vec3::ZERO,
            rest_length: 0.0,
        }
    }

    pub fn with_length(self, length: Scalar) -> Self {
        Self { rest_length: length, ..self }
    }

    pub fn with_local_pos1(self, local_pos1: Vec3) -> Self {
        Self { local_pos1, ..self }
    }

    pub fn with_local_pos2(self, local_pos2: Vec3) -> Self {
        Self { local_pos2, ..self }
    }

    pub fn apply(&self, body1: &mut RigidBody, body2: &mut RigidBody) {
        let world_pos1 = body1.local_position_2_world(self.local_pos1);
        let world_pos2 = body2.local_position_2_world(self.local_pos2);
        let delta = world_pos2 - world_pos1;
        let delta_length = delta.length();
        if delta_length < 1e-6 {
            return;
        }
        let delta_direction = delta / delta_length;
        let joint_force_magnitude = 100.0 * (delta_length - self.rest_length);
        let joint_force = delta_direction * joint_force_magnitude;
        
        body1.apply_force(joint_force, world_pos1);
        body2.apply_force(-joint_force, world_pos2);

        let relative_velocity = body2.velocity - body1.velocity;
        let damping_force_magnitude = relative_velocity * delta_direction * 10.0;
        let damping_force = delta_direction * damping_force_magnitude;
        body1.apply_force(damping_force, world_pos1);
        body2.apply_force(-damping_force, world_pos2);
    }
}

#[wasm_bindgen]
pub struct DistanceJointThreejs {
    pub id1: i32,
    pub id2: i32,
    pub local_pos1: Vec3,
    pub local_pos2: Vec3,
    pub rest_length: Scalar,
}

#[wasm_bindgen]
impl DistanceJointThreejs {
    pub fn new(id1: i32, id2: i32) -> Self {
        Self {
            id1,
            id2,
            local_pos1: Vec3::ZERO,
            local_pos2: Vec3::ZERO,
            rest_length: 0.0,
        }
    }

    pub fn with_length(self, length: Scalar) -> Self {
        Self { rest_length: length, ..self }
    }

    pub fn with_local_pos1(self, local_pos1: Vec3) -> Self {
        Self { local_pos1, ..self }
    }

    pub fn with_local_pos2(self, local_pos2: Vec3) -> Self {
        Self { local_pos2, ..self }
    }

    pub fn apply(&self, body1: &mut RigidBody, body2: &mut RigidBody) {
        let world_pos1 = body1.local_position_2_world(self.local_pos1);
        let world_pos2 = body2.local_position_2_world(self.local_pos2);
        let delta = world_pos2 - world_pos1;
        let delta_length = delta.length();
        let delta_direction = delta / delta_length;
        let joint_force_magnitude = 100.0 * (delta_length - self.rest_length);
        let joint_force = delta_direction * joint_force_magnitude;
        
        body1.apply_force(joint_force, world_pos1);
        body2.apply_force(-joint_force, world_pos2);

        let relative_velocity = body2.velocity - body1.velocity;
        let damping_force_magnitude = relative_velocity * delta_direction * 10.0;
        let damping_force = delta_direction * damping_force_magnitude;
        body1.apply_force(damping_force, world_pos1);
        body2.apply_force(-damping_force, world_pos2);
    }
}