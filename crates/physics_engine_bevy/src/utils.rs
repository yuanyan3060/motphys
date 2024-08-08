use physics_engine_core::math::Quaternion;

pub fn engine_vec3_2_bevy(value: physics_engine_core::math::Vec3) -> bevy::math::Vec3 {
    bevy::math::Vec3 {
        x: value.x,
        y: value.y,
        z: value.z
    }
}

pub fn engine_quat_2_bevy(value: physics_engine_core::math::Quaternion) -> bevy::math::Quat {
    let physics_engine_core::math::Quaternion { w, x, y, z } = value;
    bevy::math::Quat::from_xyzw(x, y, z, w)
}

pub fn bevy_vec3_2_engine(value: bevy::math::Vec3) -> physics_engine_core::math::Vec3 {
    physics_engine_core::math::Vec3 {
        x: value.x,
        y: value.y,
        z: value.z
    }
}

pub fn bevy_quat_2_engine(value: bevy::math::Quat) ->  physics_engine_core::math::Quaternion{
    let [x, y, z, w] = value.to_array();
    Quaternion::new(w, x, y, z)
}