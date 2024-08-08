use crate::math::{Matrix3x3, Scalar};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
pub trait Shape {
    // 获取惯性张量
    fn get_inertia_tensor(&self, mass: Scalar) -> Matrix3x3;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Shapes {
    Boxes(Boxes), // 暂时只实现长方体
}

impl Shape for Shapes {
    fn get_inertia_tensor(&self, mass: Scalar) -> Matrix3x3 {
        match self {
            Shapes::Boxes(boxes) => boxes.get_inertia_tensor(mass),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct Boxes {
    /// 沿x轴方向长度
    pub half_width: Scalar,
    /// 沿y轴方向长度
    pub half_height: Scalar,
    /// 沿z轴方向长度
    pub half_depth: Scalar,
}

#[wasm_bindgen]
impl Boxes {
    pub fn new(half_width: Scalar, half_height: Scalar, half_depth: Scalar) -> Self {
        Self {
            half_width,
            half_height,
            half_depth,
        }
    }

    pub fn cube(half: Scalar) -> Self {
        Self::new(half, half, half)
    }
}

impl Shape for Boxes {
    fn get_inertia_tensor(&self, mass: Scalar) -> Matrix3x3 {
        let scale = mass / 3.0;
        let compute = |m: Scalar, n: Scalar| scale * (m * m + n * n);
        Matrix3x3::new([
            [compute(self.half_height, self.half_depth), 0.0, 0.0],
            [0.0, compute(self.half_width, self.half_depth), 0.0],
            [0.0, 0.0, compute(self.half_width, self.half_height)],
        ])
    }
}

impl From<Boxes> for Shapes {
    fn from(value: Boxes) -> Self {
        Self::Boxes(value)
    }
}
