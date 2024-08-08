use crate::{
    math::{Matrix3x3, Quaternion, Scalar, Vec3},
    shape::{Shape, Shapes},
    Boxes,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[wasm_bindgen]
pub enum RigidBodyType {
    Static,
    Dynamic,
}

#[derive(Debug)]
#[cfg_attr(feature = "use_bevy", derive(bevy::prelude::Component))]
#[derive(Serialize, Deserialize)]
#[wasm_bindgen]
pub struct RigidBody {
    /// 形状
    #[wasm_bindgen(skip)]
    pub shape: Shapes,
    /// 局部坐标下的惯性张量
    #[wasm_bindgen(skip)]
    pub inertia: Matrix3x3,
    /// 局部坐标下的惯性张量的逆
    #[wasm_bindgen(skip)]
    pub inertia_inverse: Matrix3x3,
    /// 全局坐标
    pub position: Vec3,
    /// 全局速度
    #[wasm_bindgen(skip)]
    pub velocity: Vec3,
    /// 局部角速度
    #[wasm_bindgen(skip)]
    pub angular_velocity: Vec3,
    /// 质量
    pub mass: Scalar,
    /// 刚体类型
    pub body_type: RigidBodyType,
    /// 四元数
    pub quaternion: Quaternion,
    /// 合力
    pub force: Vec3,
    /// 合力矩
    pub torque: Vec3,
}
impl RigidBody {
    pub fn new(shape: impl Into<Shapes>, mass: Scalar) -> Option<Self> {
        let shape = shape.into();
        let inertia = shape.get_inertia_tensor(mass);
        Some(Self {
            shape,
            inertia,
            inertia_inverse: inertia.try_inverse()?,
            position: Vec3::ZERO,
            velocity: Vec3::ZERO,
            angular_velocity: Vec3::ZERO,
            mass,
            body_type: RigidBodyType::Dynamic,
            quaternion: Quaternion::default(),
            force: Vec3::ZERO,
            torque: Vec3::ZERO,
        })
    }
}

#[wasm_bindgen]
impl RigidBody {
    pub fn new_box(shape: Boxes, mass: Scalar) -> Option<RigidBody> {
        let inertia = shape.get_inertia_tensor(mass);
        Some(Self {
            shape: shape.into(),
            inertia,
            inertia_inverse: inertia.try_inverse()?,
            position: Vec3::ZERO,
            velocity: Vec3::ZERO,
            angular_velocity: Vec3::ZERO,
            mass,
            body_type: RigidBodyType::Dynamic,
            quaternion: Quaternion::default(),
            force: Vec3::ZERO,
            torque: Vec3::ZERO,
        })
    }

    #[inline]
    pub fn with_position(self, position: Vec3) -> Self {
        Self { position, ..self }
    }

    #[inline]
    pub fn with_quaternion(self, quaternion: Quaternion) -> Self {
        Self { quaternion, ..self }
    }

    #[inline]
    pub fn with_velocity(self, velocity: Vec3) -> Self {
        Self { velocity, ..self }
    }

    #[inline]
    pub fn with_type(self, body_type: RigidBodyType) -> Self {
        Self { body_type, ..self }
    }

    pub fn update(&mut self, dt: Scalar) {
        if self.body_type == RigidBodyType::Static {
            return;
        }
        self.velocity += self.force * dt / self.mass;
        self.position += self.velocity * dt;
        //println!("torque:{:?}", self.torque);
        self.angular_velocity += self.inertia_inverse
            * (self.torque
                - self
                    .angular_velocity
                    .cross(&(self.inertia * self.angular_velocity)))
            * dt;
        //println!("angular_velocity:{:?}", self.angular_velocity);
        let angular_velocity_quat = Quaternion {
            w: 0.0,
            x: self.angular_velocity.x,
            y: self.angular_velocity.y,
            z: self.angular_velocity.z,
        };
        self.quaternion += angular_velocity_quat * self.quaternion * (0.5 * dt);
        self.quaternion.normalize();
        self.reset_force()
    }
}

#[wasm_bindgen]
impl RigidBody {
    pub fn local_position_2_world(&self, local: Vec3) -> Vec3 {
        self.position + self.quaternion * local
    }

    pub fn apply_force(&mut self, force: Vec3, point: Vec3) {
        if self.body_type == RigidBodyType::Static {
            return;
        }
        self.force += force;
        self.torque += (point - self.position).cross(&force)
    }

    pub fn apply_torque(&mut self, torque: Vec3) {
        if self.body_type == RigidBodyType::Static {
            return;
        }
        self.torque += torque
    }

    pub fn apply_offset(&mut self, offset: Vec3) {
        if self.body_type == RigidBodyType::Static {
            return;
        }
        self.position += offset;
    }


    pub fn reset_force(&mut self) {
        self.force = Vec3::ZERO;
        self.torque = Vec3::ZERO;
    }
}
