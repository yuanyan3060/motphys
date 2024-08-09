/* tslint:disable */
/* eslint-disable */
/**
*/
export enum RigidBodyType {
  Static = 0,
  Dynamic = 1,
}
/**
*/
export class Boxes {
  free(): void;
/**
* @param {number} half_width
* @param {number} half_height
* @param {number} half_depth
* @returns {Boxes}
*/
  static new(half_width: number, half_height: number, half_depth: number): Boxes;
/**
* @param {number} half
* @returns {Boxes}
*/
  static cube(half: number): Boxes;
/**
* 沿z轴方向长度
*/
  half_depth: number;
/**
* 沿y轴方向长度
*/
  half_height: number;
/**
* 沿x轴方向长度
*/
  half_width: number;
}
/**
*/
export class DistanceJointThreejs {
  free(): void;
/**
* @param {number} id1
* @param {number} id2
* @returns {DistanceJointThreejs}
*/
  static new(id1: number, id2: number): DistanceJointThreejs;
/**
* @param {number} length
* @returns {DistanceJointThreejs}
*/
  with_length(length: number): DistanceJointThreejs;
/**
* @param {Vec3} local_pos1
* @returns {DistanceJointThreejs}
*/
  with_local_pos1(local_pos1: Vec3): DistanceJointThreejs;
/**
* @param {Vec3} local_pos2
* @returns {DistanceJointThreejs}
*/
  with_local_pos2(local_pos2: Vec3): DistanceJointThreejs;
/**
* @param {RigidBody} body1
* @param {RigidBody} body2
*/
  apply(body1: RigidBody, body2: RigidBody): void;
/**
*/
  id1: number;
/**
*/
  id2: number;
/**
*/
  local_pos1: Vec3;
/**
*/
  local_pos2: Vec3;
/**
*/
  rest_length: number;
}
/**
*/
export class Quaternion {
  free(): void;
/**
*/
  w: number;
/**
*/
  x: number;
/**
*/
  y: number;
/**
*/
  z: number;
}
/**
*/
export class RigidBody {
  free(): void;
/**
* @param {Boxes} shape
* @param {number} mass
* @returns {RigidBody | undefined}
*/
  static new_box(shape: Boxes, mass: number): RigidBody | undefined;
/**
* @param {Vec3} position
* @returns {RigidBody}
*/
  with_position(position: Vec3): RigidBody;
/**
* @param {Vec3} velocity
* @returns {RigidBody}
*/
  with_velocity(velocity: Vec3): RigidBody;
/**
* @param {RigidBodyType} body_type
* @returns {RigidBody}
*/
  with_type(body_type: RigidBodyType): RigidBody;
/**
* @param {number} dt
*/
  update(dt: number): void;
/**
* @param {Vec3} local
* @returns {Vec3}
*/
  local_position_2_world(local: Vec3): Vec3;
/**
* @param {Vec3} force
* @param {Vec3} point
*/
  apply_force(force: Vec3, point: Vec3): void;
/**
*/
  reset_force(): void;
/**
* 刚体类型
*/
  body_type: RigidBodyType;
/**
* 合力
*/
  force: Vec3;
/**
* 质量
*/
  mass: number;
/**
* 全局坐标
*/
  position: Vec3;
/**
* 四元数
*/
  quaternion: Quaternion;
/**
* 合力矩
*/
  torque: Vec3;
}
/**
*/
export class Vec3 {
  free(): void;
/**
* @param {number} x
* @param {number} y
* @param {number} z
* @returns {Vec3}
*/
  static new(x: number, y: number, z: number): Vec3;
/**
* @param {Vec3} other
* @returns {Vec3}
*/
  cross(other: Vec3): Vec3;
/**
* @returns {number}
*/
  length(): number;
/**
*/
  x: number;
/**
*/
  y: number;
/**
*/
  z: number;
}
