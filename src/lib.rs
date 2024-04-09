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
    fn new(min: Vector3<f32>, max: Vector3<f32>) -> Self {
        Self { min, max }
    }
}

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
    fn sphere_to() {
        let s = Sphere {
            pos: Vector3::new(0., 3., -1.5),
            radius: 1.0,
        };
        let b = s.get_bounds();

        assert_eq!(b, AABB::new(
            Vector3::new(-1.0, 2.0, -2.5),
            Vector3::new(1.0, 4.0, -0.5),
        ));

        println!("{:?}", s);
        println!("{:?}", b);
    }
}
