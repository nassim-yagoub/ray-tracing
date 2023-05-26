use crate::vec3::{Point3, Vec3};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::*;

    macro_rules! assert_vec3_equal {
        ($expected:expr, $actual:expr) => {
            let tolerance = 0.0001;
            assert_relative_eq!($expected.x, $actual.x, epsilon = tolerance);
            assert_relative_eq!($expected.y, $actual.y, epsilon = tolerance);
            assert_relative_eq!($expected.z, $actual.z, epsilon = tolerance);
        };
    }

    #[test]
    fn at() {
        let origin = Point3::new(2.0, 4.0, 7.0);
        let direction = Vec3::new(1.0, 2.0, 3.0);
        let ray = Ray::new(origin, direction);
        let result = ray.at(5.0);
        let expected = Vec3::new(7.0, 14.0, 22.0);
        assert_vec3_equal!(expected, result);
    }
}
