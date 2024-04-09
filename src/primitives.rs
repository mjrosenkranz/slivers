use nalgebra::Vector3;
use crate::bounding_box::{AABB, Bounds};

#[derive(Debug)]
pub struct Sphere {
    pos: Vector3<f32>,
    radius: f32,
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
        let s = Sphere {
            pos: Vector3::new(0., 3., -1.5),
            radius: 1.0,
        };
        let b = s.get_bounds();

        assert_eq!(b, AABB::new(
            Vector3::new(-1.0, 2.0, -2.5),
            Vector3::new(1.0, 4.0, -0.5),
        ));
    }
}
