use crate::{
    math::{Quaternion, Scalar, Vec3},
    Boxes, RigidBody, Shapes,
};

pub struct Collision {
    face: Vec3,
    overlap: Scalar,
}

/// 碰撞检测 暂时只实现长方体的
pub fn collision_check(body1: &RigidBody, body2: &RigidBody) -> Option<Collision> {
    #[allow(irrefutable_let_patterns)]
    let Shapes::Boxes(box1) = body1.shape else {
        return None;
    };
    #[allow(irrefutable_let_patterns)]
    let Shapes::Boxes(box2) = body2.shape else {
        return None;
    };
    let box1_vertex = get_box_vertex(box1, body1.position, body1.quaternion);
    let box2_vertex = get_box_vertex(box2, body2.position, body2.quaternion);
    let face1 = get_box_face_normal(body1.quaternion);
    let face2 = get_box_face_normal(body2.quaternion);
    let face3 = face1
        .iter()
        .flat_map(|x| face2.iter().filter_map(|y| x.cross(y).try_normalize()));
    let faces = face1.into_iter().chain(face2).chain(face3);
    let mut ret: Option<Collision> = None;
    for face in faces {
        let (min1, max1) = projection(&box1_vertex, face);
        let (min2, max2) = projection(&box2_vertex, face);
        let overlap = calculate_overlap(min1, max1, min2, max2)?;
        let need_update = ret
            .as_ref()
            .map(|last| overlap < last.overlap)
            .unwrap_or(true);
        if need_update {
            ret = Some(Collision {
                face,
                overlap,
            })
        }
    }
    ret
}

// 获取长方体表面的法向量
fn get_box_face_normal(quaternion: Quaternion) -> [Vec3; 3] {
    [
        quaternion.rotate_vec3(Vec3::new(1.0, 0.0, 0.0)),
        quaternion.rotate_vec3(Vec3::new(0.0, 1.0, 0.0)),
        quaternion.rotate_vec3(Vec3::new(0.0, 0.0, 1.0)),
    ]
}

// 获取长方体顶点坐标
fn get_box_vertex(boxes: Boxes, position: Vec3, quaternion: Quaternion) -> [Vec3; 8] {
    let mut ret = [
        Vec3::new(boxes.half_width, boxes.half_height, boxes.half_depth),
        Vec3::new(boxes.half_width, boxes.half_height, -boxes.half_depth),
        Vec3::new(boxes.half_width, -boxes.half_height, -boxes.half_depth),
        Vec3::new(boxes.half_width, -boxes.half_height, boxes.half_depth),
        Vec3::new(-boxes.half_width, -boxes.half_height, boxes.half_depth),
        Vec3::new(-boxes.half_width, -boxes.half_height, -boxes.half_depth),
        Vec3::new(-boxes.half_width, boxes.half_height, -boxes.half_depth),
        Vec3::new(-boxes.half_width, boxes.half_height, boxes.half_depth),
    ];
    ret.iter_mut().for_each(|x| *x = quaternion.rotate_vec3(*x));
    ret.iter_mut().for_each(|x| *x += position);
    ret
}

// 获取长方体顶点投影的最小值和最大值
fn projection(box_vertex: &[Vec3; 8], face: Vec3) -> (Scalar, Scalar) {
    let mut min = Scalar::MAX;
    let mut max = Scalar::MIN;
    for vertex in box_vertex {
        let projection = *vertex * face;
        min = min.min(projection);
        max = max.max(projection);
    }
    (min, max)
}

// 获取重叠范围 为None则表示不重叠
fn calculate_overlap(min1: f32, max1: f32, min2: f32, max2: f32) -> Option<f32> {
    if max1 < min2 || max2 < min1 {
        return None; // 没有重叠
    }
    let overlap = (max1.min(max2) - min1.max(min2)).abs();
    Some(overlap)
}

pub fn resolve_collision(body1: &mut RigidBody, body2: &mut RigidBody) {
    let Some(collision) = collision_check(body1, body2) else {
        return;
    };
    let mut face = collision.face;
    let delta = body2.position - body1.position;
    if (delta * face) < 0.0 {
        face = -face
    };
    let offset = face * collision.overlap;
    body1.apply_offset(-offset);
    body2.apply_offset(offset);
    body1.velocity *= -0.8;
    body2.velocity *= -0.8;
}
