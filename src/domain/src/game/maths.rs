use std::{fmt::Display};

#[derive(Copy, Clone, Debug)]
pub struct Matrix34 {
    pub vec0: Vector4,
    pub vec1: Vector4,
    pub vec2: Vector4,
}

#[derive(Copy, Clone, Debug)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vector4 {
        Vector4 { x, y, z, w }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x.round(), self.y.round(), self.z.round())
    }
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn flip(old: Vector3) -> Vector3 {
        Vector3::new(old.z, old.x, old.y)
    }

    pub fn default() -> Vector3 {
        Vector3::new(0.0, 0.0, 0.0)
    }

    pub fn distance(self, other: Vector3) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)).sqrt()
    }

    pub fn dot_product(&self, other: Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Display for Vector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x.round(), self.y.round())
    }
}

impl Vector2 {
    pub const ZERO: Vector2 = Vector2 { x: 0.0, y: 0.0 };

    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x, y }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix44 {
    pub m11: f32,
    pub m12: f32,
    pub m13: f32,
    pub m14: f32,
    pub m21: f32,
    pub m22: f32,
    pub m23: f32,
    pub m24: f32,
    pub m31: f32,
    pub m32: f32,
    pub m33: f32,
    pub m34: f32,
    pub m41: f32,
    pub m42: f32,
    pub m43: f32,
    pub m44: f32,
}

impl Matrix44 {
    pub fn transpose(&self) -> Matrix44 {
        Matrix44 {
            m11: self.m11,
            m12: self.m21,
            m13: self.m31,
            m14: self.m41,
            m21: self.m12,
            m22: self.m22,
            m23: self.m32,
            m24: self.m42,
            m31: self.m13,
            m32: self.m23,
            m33: self.m33,
            m34: self.m43,
            m41: self.m14,
            m42: self.m24,
            m43: self.m34,
            m44: self.m44,
        }
    }
}