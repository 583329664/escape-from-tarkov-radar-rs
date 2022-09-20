use crate::game::maths::Matrix44;

#[derive(Clone, Debug, PartialEq)]
pub struct Camera {
    pub matrix: Matrix44,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub camera_type: CameraType
}

#[derive(Clone, Debug, PartialEq)]
pub enum CameraType {
    FpsCamera,
    OpticCamera,
}
