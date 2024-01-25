use glam::{Mat2, Mat3, Mat4, Vec2, Vec3};

pub mod vec {
    pub use crate::{Vector2, Vector3};
}

pub mod mat {
    pub use crate::{Matrix2, Matrix3, Matrix4};
}

pub type Vector2 = Vec2;
pub type Vector3 = Vec3;
pub type Matrix2 = Mat2;
pub type Matrix3 = Mat3;
pub type Matrix4 = Mat4;