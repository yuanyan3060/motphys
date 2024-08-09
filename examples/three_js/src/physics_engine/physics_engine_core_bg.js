let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}


const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachedUint8Memory0 = null;

function getUint8Memory0() {
    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}
/**
*/
export const RigidBodyType = Object.freeze({ Static:0,"0":"Static",Dynamic:1,"1":"Dynamic", });

const BoxesFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_boxes_free(ptr >>> 0));
/**
*/
export class Boxes {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Boxes.prototype);
        obj.__wbg_ptr = ptr;
        BoxesFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        BoxesFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_boxes_free(ptr);
    }
    /**
    * 沿x轴方向长度
    * @returns {number}
    */
    get half_width() {
        const ret = wasm.__wbg_get_boxes_half_width(this.__wbg_ptr);
        return ret;
    }
    /**
    * 沿x轴方向长度
    * @param {number} arg0
    */
    set half_width(arg0) {
        wasm.__wbg_set_boxes_half_width(this.__wbg_ptr, arg0);
    }
    /**
    * 沿y轴方向长度
    * @returns {number}
    */
    get half_height() {
        const ret = wasm.__wbg_get_boxes_half_height(this.__wbg_ptr);
        return ret;
    }
    /**
    * 沿y轴方向长度
    * @param {number} arg0
    */
    set half_height(arg0) {
        wasm.__wbg_set_boxes_half_height(this.__wbg_ptr, arg0);
    }
    /**
    * 沿z轴方向长度
    * @returns {number}
    */
    get half_depth() {
        const ret = wasm.__wbg_get_boxes_half_depth(this.__wbg_ptr);
        return ret;
    }
    /**
    * 沿z轴方向长度
    * @param {number} arg0
    */
    set half_depth(arg0) {
        wasm.__wbg_set_boxes_half_depth(this.__wbg_ptr, arg0);
    }
    /**
    * @param {number} half_width
    * @param {number} half_height
    * @param {number} half_depth
    * @returns {Boxes}
    */
    static new(half_width, half_height, half_depth) {
        const ret = wasm.boxes_new(half_width, half_height, half_depth);
        return Boxes.__wrap(ret);
    }
    /**
    * @param {number} half
    * @returns {Boxes}
    */
    static cube(half) {
        const ret = wasm.boxes_cube(half);
        return Boxes.__wrap(ret);
    }
}

const DistanceJointThreejsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_distancejointthreejs_free(ptr >>> 0));
/**
*/
export class DistanceJointThreejs {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(DistanceJointThreejs.prototype);
        obj.__wbg_ptr = ptr;
        DistanceJointThreejsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DistanceJointThreejsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_distancejointthreejs_free(ptr);
    }
    /**
    * @returns {number}
    */
    get id1() {
        const ret = wasm.__wbg_get_distancejointthreejs_id1(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set id1(arg0) {
        wasm.__wbg_set_distancejointthreejs_id1(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get id2() {
        const ret = wasm.__wbg_get_distancejointthreejs_id2(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set id2(arg0) {
        wasm.__wbg_set_distancejointthreejs_id2(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {Vec3}
    */
    get local_pos1() {
        const ret = wasm.__wbg_get_distancejointthreejs_local_pos1(this.__wbg_ptr);
        return Vec3.__wrap(ret);
    }
    /**
    * @param {Vec3} arg0
    */
    set local_pos1(arg0) {
        _assertClass(arg0, Vec3);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_distancejointthreejs_local_pos1(this.__wbg_ptr, ptr0);
    }
    /**
    * @returns {Vec3}
    */
    get local_pos2() {
        const ret = wasm.__wbg_get_distancejointthreejs_local_pos2(this.__wbg_ptr);
        return Vec3.__wrap(ret);
    }
    /**
    * @param {Vec3} arg0
    */
    set local_pos2(arg0) {
        _assertClass(arg0, Vec3);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_distancejointthreejs_local_pos2(this.__wbg_ptr, ptr0);
    }
    /**
    * @returns {number}
    */
    get rest_length() {
        const ret = wasm.__wbg_get_distancejointthreejs_rest_length(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set rest_length(arg0) {
        wasm.__wbg_set_distancejointthreejs_rest_length(this.__wbg_ptr, arg0);
    }
    /**
    * @param {number} id1
    * @param {number} id2
    * @returns {DistanceJointThreejs}
    */
    static new(id1, id2) {
        const ret = wasm.distancejointthreejs_new(id1, id2);
        return DistanceJointThreejs.__wrap(ret);
    }
    /**
    * @param {number} length
    * @returns {DistanceJointThreejs}
    */
    with_length(length) {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.distancejointthreejs_with_length(ptr, length);
        return DistanceJointThreejs.__wrap(ret);
    }
    /**
    * @param {Vec3} local_pos1
    * @returns {DistanceJointThreejs}
    */
    with_local_pos1(local_pos1) {
        const ptr = this.__destroy_into_raw();
        _assertClass(local_pos1, Vec3);
        var ptr0 = local_pos1.__destroy_into_raw();
        const ret = wasm.distancejointthreejs_with_local_pos1(ptr, ptr0);
        return DistanceJointThreejs.__wrap(ret);
    }
    /**
    * @param {Vec3} local_pos2
    * @returns {DistanceJointThreejs}
    */
    with_local_pos2(local_pos2) {
        const ptr = this.__destroy_into_raw();
        _assertClass(local_pos2, Vec3);
        var ptr0 = local_pos2.__destroy_into_raw();
        const ret = wasm.distancejointthreejs_with_local_pos2(ptr, ptr0);
        return DistanceJointThreejs.__wrap(ret);
    }
    /**
    * @param {RigidBody} body1
    * @param {RigidBody} body2
    */
    apply(body1, body2) {
        _assertClass(body1, RigidBody);
        _assertClass(body2, RigidBody);
        wasm.distancejointthreejs_apply(this.__wbg_ptr, body1.__wbg_ptr, body2.__wbg_ptr);
    }
}

const QuaternionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_quaternion_free(ptr >>> 0));
/**
*/
export class Quaternion {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Quaternion.prototype);
        obj.__wbg_ptr = ptr;
        QuaternionFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        QuaternionFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_quaternion_free(ptr);
    }
    /**
    * @returns {number}
    */
    get w() {
        const ret = wasm.__wbg_get_quaternion_w(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set w(arg0) {
        wasm.__wbg_set_quaternion_w(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get x() {
        const ret = wasm.__wbg_get_quaternion_x(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set x(arg0) {
        wasm.__wbg_set_quaternion_x(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get y() {
        const ret = wasm.__wbg_get_quaternion_y(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set y(arg0) {
        wasm.__wbg_set_quaternion_y(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get z() {
        const ret = wasm.__wbg_get_quaternion_z(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set z(arg0) {
        wasm.__wbg_set_quaternion_z(this.__wbg_ptr, arg0);
    }
}

const RigidBodyFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_rigidbody_free(ptr >>> 0));
/**
*/
export class RigidBody {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(RigidBody.prototype);
        obj.__wbg_ptr = ptr;
        RigidBodyFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        RigidBodyFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_rigidbody_free(ptr);
    }
    /**
    * 全局坐标
    * @returns {Vec3}
    */
    get position() {
        const ret = wasm.__wbg_get_rigidbody_position(this.__wbg_ptr);
        return Vec3.__wrap(ret);
    }
    /**
    * 全局坐标
    * @param {Vec3} arg0
    */
    set position(arg0) {
        _assertClass(arg0, Vec3);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_rigidbody_position(this.__wbg_ptr, ptr0);
    }
    /**
    * 质量
    * @returns {number}
    */
    get mass() {
        const ret = wasm.__wbg_get_rigidbody_mass(this.__wbg_ptr);
        return ret;
    }
    /**
    * 质量
    * @param {number} arg0
    */
    set mass(arg0) {
        wasm.__wbg_set_rigidbody_mass(this.__wbg_ptr, arg0);
    }
    /**
    * 刚体类型
    * @returns {RigidBodyType}
    */
    get body_type() {
        const ret = wasm.__wbg_get_rigidbody_body_type(this.__wbg_ptr);
        return ret;
    }
    /**
    * 刚体类型
    * @param {RigidBodyType} arg0
    */
    set body_type(arg0) {
        wasm.__wbg_set_rigidbody_body_type(this.__wbg_ptr, arg0);
    }
    /**
    * 四元数
    * @returns {Quaternion}
    */
    get quaternion() {
        const ret = wasm.__wbg_get_rigidbody_quaternion(this.__wbg_ptr);
        return Quaternion.__wrap(ret);
    }
    /**
    * 四元数
    * @param {Quaternion} arg0
    */
    set quaternion(arg0) {
        _assertClass(arg0, Quaternion);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_rigidbody_quaternion(this.__wbg_ptr, ptr0);
    }
    /**
    * 合力
    * @returns {Vec3}
    */
    get force() {
        const ret = wasm.__wbg_get_rigidbody_force(this.__wbg_ptr);
        return Vec3.__wrap(ret);
    }
    /**
    * 合力
    * @param {Vec3} arg0
    */
    set force(arg0) {
        _assertClass(arg0, Vec3);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_rigidbody_force(this.__wbg_ptr, ptr0);
    }
    /**
    * 合力矩
    * @returns {Vec3}
    */
    get torque() {
        const ret = wasm.__wbg_get_rigidbody_torque(this.__wbg_ptr);
        return Vec3.__wrap(ret);
    }
    /**
    * 合力矩
    * @param {Vec3} arg0
    */
    set torque(arg0) {
        _assertClass(arg0, Vec3);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_rigidbody_torque(this.__wbg_ptr, ptr0);
    }
    /**
    * @param {Boxes} shape
    * @param {number} mass
    * @returns {RigidBody | undefined}
    */
    static new_box(shape, mass) {
        _assertClass(shape, Boxes);
        var ptr0 = shape.__destroy_into_raw();
        const ret = wasm.rigidbody_new_box(ptr0, mass);
        return ret === 0 ? undefined : RigidBody.__wrap(ret);
    }
    /**
    * @param {Vec3} position
    * @returns {RigidBody}
    */
    with_position(position) {
        const ptr = this.__destroy_into_raw();
        _assertClass(position, Vec3);
        var ptr0 = position.__destroy_into_raw();
        const ret = wasm.rigidbody_with_position(ptr, ptr0);
        return RigidBody.__wrap(ret);
    }
    /**
    * @param {Vec3} velocity
    * @returns {RigidBody}
    */
    with_velocity(velocity) {
        const ptr = this.__destroy_into_raw();
        _assertClass(velocity, Vec3);
        var ptr0 = velocity.__destroy_into_raw();
        const ret = wasm.rigidbody_with_velocity(ptr, ptr0);
        return RigidBody.__wrap(ret);
    }
    /**
    * @param {RigidBodyType} body_type
    * @returns {RigidBody}
    */
    with_type(body_type) {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.rigidbody_with_type(ptr, body_type);
        return RigidBody.__wrap(ret);
    }
    /**
    * @param {number} dt
    */
    update(dt) {
        wasm.rigidbody_update(this.__wbg_ptr, dt);
    }
    /**
    * @param {Vec3} local
    * @returns {Vec3}
    */
    local_position_2_world(local) {
        _assertClass(local, Vec3);
        var ptr0 = local.__destroy_into_raw();
        const ret = wasm.rigidbody_local_position_2_world(this.__wbg_ptr, ptr0);
        return Vec3.__wrap(ret);
    }
    /**
    * @param {Vec3} force
    * @param {Vec3} point
    */
    apply_force(force, point) {
        _assertClass(force, Vec3);
        var ptr0 = force.__destroy_into_raw();
        _assertClass(point, Vec3);
        var ptr1 = point.__destroy_into_raw();
        wasm.rigidbody_apply_force(this.__wbg_ptr, ptr0, ptr1);
    }
    /**
    */
    reset_force() {
        wasm.rigidbody_reset_force(this.__wbg_ptr);
    }
}

const Vec3Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_vec3_free(ptr >>> 0));
/**
*/
export class Vec3 {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Vec3.prototype);
        obj.__wbg_ptr = ptr;
        Vec3Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        Vec3Finalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_vec3_free(ptr);
    }
    /**
    * @returns {number}
    */
    get x() {
        const ret = wasm.__wbg_get_quaternion_w(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set x(arg0) {
        wasm.__wbg_set_quaternion_w(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get y() {
        const ret = wasm.__wbg_get_quaternion_x(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set y(arg0) {
        wasm.__wbg_set_quaternion_x(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get z() {
        const ret = wasm.__wbg_get_quaternion_y(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set z(arg0) {
        wasm.__wbg_set_quaternion_y(this.__wbg_ptr, arg0);
    }
    /**
    * @param {number} x
    * @param {number} y
    * @param {number} z
    * @returns {Vec3}
    */
    static new(x, y, z) {
        const ret = wasm.vec3_new(x, y, z);
        return Vec3.__wrap(ret);
    }
    /**
    * @param {Vec3} other
    * @returns {Vec3}
    */
    cross(other) {
        const ptr = this.__destroy_into_raw();
        _assertClass(other, Vec3);
        const ret = wasm.vec3_cross(ptr, other.__wbg_ptr);
        return Vec3.__wrap(ret);
    }
    /**
    * @returns {number}
    */
    length() {
        const ret = wasm.vec3_length(this.__wbg_ptr);
        return ret;
    }
}

export function __wbindgen_throw(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

