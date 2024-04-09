use std::ops::Add;

use nalgebra::Vector3;

pub trait Bounds {
    fn get_bounds(&self) -> AABB;
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct AABB {
    min: Vector3<f32>,
    max: Vector3<f32>,
}

impl AABB {
    pub fn new(min: Vector3<f32>, max: Vector3<f32>) -> Self {
        Self { min, max }
    }

    pub fn volume(self) -> f32 {
        let mag = self.max - self.min;
        mag.x * mag.y * mag.z
    }
}

impl Add for AABB {
    type Output = AABB;

    fn add(self, rhs: AABB) -> AABB {
        AABB::new(
            Vector3::new(
                f32::min(self.min.x, rhs.min.x),
                f32::min(self.min.y, rhs.min.y),
                f32::min(self.min.z, rhs.min.z),
            ),
            Vector3::new(
                f32::max(self.max.x, rhs.max.x),
                f32::max(self.max.y, rhs.max.y),
                f32::max(self.max.z, rhs.max.z),
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let bb = AABB::new(Vector3::new(0., 0., 0.), Vector3::new(1., 1., 1.));
        assert_eq!(bb.volume(), 1.);
    }

    #[test]
    fn vol() {
        let bb = AABB::new(Vector3::new(0., 0., -3.), Vector3::new(3., 4., -1.));
        assert_eq!(bb.volume(), 24.);
    }

    #[test]
    fn add_bb() {
        let bb1 = AABB::new(Vector3::new(-1., -2., -3.), Vector3::new(0., 0., 0.));
        let bb2 = AABB::new(Vector3::new(0., 0., 0.), Vector3::new(4., 5., 6.));
        let bb3 = bb1 + bb2;

        assert_eq!(
            bb3,
            AABB::new(Vector3::new(-1., -2., -3.), Vector3::new(4., 5., 6.))
        );
    }
}
