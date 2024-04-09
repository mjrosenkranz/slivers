use crate::bounding::{Bounds, AABB};
use nalgebra::Vector3;

#[derive(Debug)]
pub struct Sphere {
    pos: Vector3<f32>,
    radius: f32,
}

impl Sphere {
    fn new(pos: Vector3<f32>, radius: f32) -> Self {
        Self { pos, radius }
    }
}

impl Bounds for Sphere {
    fn get_bounds(&self) -> AABB {
        let r = Vector3::new(self.radius, self.radius, self.radius);
        AABB::new(self.pos - r, self.pos + r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sphere() {
        let s = Sphere::new(Vector3::new(0., 3., -1.5), 1.0);
        let b = s.get_bounds();

        assert_eq!(
            b,
            AABB::new(Vector3::new(-1.0, 2.0, -2.5), Vector3::new(1.0, 4.0, -0.5),)
        );
    }
}
