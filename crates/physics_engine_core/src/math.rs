use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};
use wasm_bindgen::prelude::*;

pub type Scalar = f32;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct Vec3 {
    pub x: Scalar,
    pub y: Scalar,
    pub z: Scalar,
}

impl Vec3 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
}

#[wasm_bindgen]
impl Vec3 {
    pub fn new(x: Scalar, y: Scalar, z: Scalar) -> Self {
        Self { x, y, z }
    }

    pub fn cross(self, other: &Vec3) -> Self {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn length(&self) -> Scalar {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(self) -> Self {
        self / self.length()
    }

    pub fn try_normalize(self) -> Option<Vec3> {
        let len = self.length();
        if len < 1e-6 {
            return None;
        }
        Some(self / self.length())
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self::ZERO
    }
}
impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<Scalar> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<Scalar> for Vec3 {
    fn mul_assign(&mut self, rhs: Scalar) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Mul for Vec3 {
    type Output = Scalar;

    fn mul(self, rhs: Vec3) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Div<Scalar> for Vec3 {
    type Output = Self;

    fn div(self, rhs: Scalar) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Matrix3x3 {
    pub inner: [[Scalar; 3]; 3],
}

impl Matrix3x3 {
    pub fn new(inner: [[Scalar; 3]; 3]) -> Self {
        Self { inner }
    }

    pub fn transpose(&self) -> Self {
        let mut result = [[0.0; 3]; 3];
        #[allow(clippy::needless_range_loop)]
        for i in 0..3 {
            #[allow(clippy::needless_range_loop)]
            for j in 0..3 {
                result[j][i] = self.inner[i][j];
            }
        }
        Self::new(result)
    }

    pub fn determinant(&self) -> Scalar {
        let a = self.inner[0][0];
        let b = self.inner[0][1];
        let c = self.inner[0][2];
        let d = self.inner[1][0];
        let e = self.inner[1][1];
        let f = self.inner[1][2];
        let g = self.inner[2][0];
        let h = self.inner[2][1];
        let i = self.inner[2][2];

        a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g)
    }

    pub fn adjoint(&self) -> Self {
        Self {
            inner: [
                [
                    self.inner[1][1] * self.inner[2][2] - self.inner[1][2] * self.inner[2][1],
                    self.inner[0][2] * self.inner[2][1] - self.inner[0][1] * self.inner[2][2],
                    self.inner[0][1] * self.inner[1][2] - self.inner[0][2] * self.inner[1][1],
                ],
                [
                    self.inner[1][2] * self.inner[2][0] - self.inner[1][0] * self.inner[2][2],
                    self.inner[0][0] * self.inner[2][2] - self.inner[0][2] * self.inner[2][0],
                    self.inner[0][2] * self.inner[1][0] - self.inner[0][0] * self.inner[1][2],
                ],
                [
                    self.inner[1][0] * self.inner[2][1] - self.inner[1][1] * self.inner[2][0],
                    self.inner[0][1] * self.inner[2][0] - self.inner[0][0] * self.inner[2][1],
                    self.inner[0][0] * self.inner[1][1] - self.inner[0][1] * self.inner[1][0],
                ],
            ],
        }
    }

    pub fn try_inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if det.abs() < 1e-10 {
            return None;
        }

        let adj = self.adjoint();
        let inv = Self {
            inner: [
                [
                    adj.inner[0][0] / det,
                    adj.inner[0][1] / det,
                    adj.inner[0][2] / det,
                ],
                [
                    adj.inner[1][0] / det,
                    adj.inner[1][1] / det,
                    adj.inner[1][2] / det,
                ],
                [
                    adj.inner[2][0] / det,
                    adj.inner[2][1] / det,
                    adj.inner[2][2] / det,
                ],
            ],
        };
        Some(inv)
    }
}

impl Mul for Matrix3x3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = [[0.0; 3]; 3];
        #[allow(clippy::needless_range_loop)]
        for i in 0..3 {
            #[allow(clippy::needless_range_loop)]
            for j in 0..3 {
                result[i][j] = self.inner[i][0] * rhs.inner[0][j]
                    + self.inner[i][1] * rhs.inner[1][j]
                    + self.inner[i][2] * rhs.inner[2][j];
            }
        }

        Self::new(result)
    }
}

impl Mul<Vec3> for Matrix3x3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.inner[0][0] * rhs.x + self.inner[0][1] * rhs.y + self.inner[0][2] * rhs.z,
            y: self.inner[1][0] * rhs.x + self.inner[1][1] * rhs.y + self.inner[1][2] * rhs.z,
            z: self.inner[2][0] * rhs.x + self.inner[2][1] * rhs.y + self.inner[2][2] * rhs.z,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct Quaternion {
    pub w: Scalar,
    pub x: Scalar,
    pub y: Scalar,
    pub z: Scalar,
}

impl Quaternion {
    pub fn new(w: f32, x: f32, y: f32, z: f32) -> Self {
        Quaternion { w, x, y, z }
    }

    pub fn from_vec3(vec3: Vec3) -> Self {
        Self {
            w: 0.0,
            x: vec3.x,
            y: vec3.y,
            z: vec3.z,
        }
    }

    pub fn into_vec3(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }

    pub fn magnitude(&self) -> Scalar {
        self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(&mut self) {
        let magnitude = self.magnitude();
        self.w /= magnitude;
        self.x /= magnitude;
        self.y /= magnitude;
        self.z /= magnitude;
    }

    pub fn conjugate(&self) -> Self {
        Quaternion {
            w: self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub fn rotation_matrix(&self) -> Matrix3x3 {
        let Self { w, x, y, z } = *self;
        Matrix3x3::new([
            [
                1.0 - 2.0 * (y * y + z * z),
                2.0 * (x * y - z * w),
                2.0 * (x * z + y * w),
            ],
            [
                2.0 * (x * y + z * w),
                1.0 - 2.0 * (x * x + z * z),
                2.0 * (y * z - x * w),
            ],
            [
                2.0 * (x * z - y * w),
                2.0 * (y * z + x * w),
                1.0 - 2.0 * (x * x + y * y),
            ],
        ])
    }

    pub fn rotate_vec3(&self, vec3: Vec3) -> Vec3 {
        let q_conjugate = self.conjugate();
        let v_as_quat = Quaternion::from_vec3(vec3);
        (*self * v_as_quat * q_conjugate).into_vec3()
    }
}

impl Default for Quaternion {
    fn default() -> Self {
        Self {
            w: 1.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Add for Quaternion {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Quaternion {
            w: self.w + other.w,
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Quaternion {
    fn add_assign(&mut self, rhs: Self) {
        self.w += rhs.w;
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl Mul for Quaternion {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Quaternion {
            w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
            x: self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
            y: self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x,
            z: self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w,
        }
    }
}

impl Mul<Vec3> for Quaternion {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Self::Output {
        let q_vec = Quaternion::new(0.0, vec.x, vec.y, vec.z);
        let q_conjugate = self.conjugate();
        let result = self * q_vec * q_conjugate;
        Vec3::new(result.x, result.y, result.z)
    }
}

impl Mul<Scalar> for Quaternion {
    type Output = Quaternion;

    fn mul(self, other: Scalar) -> Self::Output {
        Self {
            w: self.w * other,
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Plane {
    pub normal: Vec3,
    pub point: Vec3
}
