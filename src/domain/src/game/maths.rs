use std::{arch::x86_64::{__m128, __m128i, _mm_castps_si128, _mm_set_ps}, fmt::Display};

use bincode::Decode;

#[derive(Copy, Clone, Debug, Decode)]
pub struct Matrix34 {
    pub vec0: Vector4,
    pub vec1: Vector4,
    pub vec2: Vector4,
}

#[derive(Copy, Clone, Debug, Decode)]
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

    pub fn into_m128i(self) -> __m128i {
        unsafe { _mm_castps_si128(_mm_set_ps(self.w, self.z, self.y, self.x)) }
    }

    pub fn into_m128(self) -> __m128 {
        unsafe { _mm_set_ps(self.w, self.z, self.y, self.x) }
    }
}

#[derive(Copy, Clone, Debug, Decode, PartialEq)]
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
}
