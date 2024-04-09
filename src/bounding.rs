use nalgebra::Vector3;

pub trait Bounds {
    fn get_bounds(&self) -> AABB;
}

#[derive(Debug, PartialEq)]
pub struct AABB {
    min: Vector3<f32>,
    max: Vector3<f32>,
}

impl AABB {
    pub fn new(min: Vector3<f32>, max: Vector3<f32>) -> Self {
        Self { min, max }
    }
}

