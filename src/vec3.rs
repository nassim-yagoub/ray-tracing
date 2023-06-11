use core::fmt;
use rand::Rng;
use std::ops::{Add, Div, Mul, Neg, Sub};

pub type Point3 = Vec3;

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, t: f64) -> Vec3 {
        Vec3 {
            x: t * self.x,
            y: t * self.y,
            z: t * self.z,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, vector: Vec3) -> Vec3 {
        Vec3 {
            x: self * vector.x,
            y: self * vector.y,
            z: self * vector.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, t: f64) -> Self::Output {
        Vec3 {
            x: self.x / t,
            y: self.y / t,
            z: self.z / t,
        }
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn random() -> Vec3 {
        let mut rng = rand::thread_rng();

        Vec3 {
            x: rng.gen_range(-1.0..1.0),
            y: rng.gen_range(-1.0..1.0),
            z: rng.gen_range(-1.0..1.0),
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        let mut vector = Self::random();
        while vector.length_squared() > 1.0 {
            vector = Self::random();
        }
        return vector;
    }

    pub fn random_unit_vector() -> Vec3 {
        return Self::random_in_unit_sphere().unit_vector();
    }

    pub fn random_in_hemishpere(normal: Vec3) -> Vec3 {
        let in_unit_sphere = Self::random_in_unit_sphere();

        if in_unit_sphere.dot(normal) > 0.0 {
            return in_unit_sphere;
        } else {
            return -in_unit_sphere;
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }

    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
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
    fn negation() {
        let vector = Vec3::new(1.0, 2.5, 4.0);
        let result = -vector;
        let expected = Vec3::new(-1.0, -2.5, -4.0);
        assert_vec3_equal!(expected, result);
    }

    #[test]
    fn addition() {
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        let vector2 = Vec3::new(4.0, 5.0, 6.0);
        let result = vector1 + vector2;
        let expected = Vec3::new(5.0, 7.0, 9.0);
        assert_vec3_equal!(expected, result);
    }

    #[test]
    fn substraction() {
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        let vector2 = Vec3::new(6.0, 5.0, 4.0);
        let result = vector1 - vector2;
        let expected = Vec3::new(-5.0, -3.0, -1.0);
        assert_vec3_equal!(expected, result);
    }

    #[test]
    fn multiplication() {
        let vector = Vec3::new(1.0, 2.0, 3.0);
        let factor = 3.0;
        let result1 = factor * vector;
        let result2 = vector * factor;
        let expected = Vec3::new(3.0, 6.0, 9.0);
        assert_vec3_equal!(expected, result1);
        assert_vec3_equal!(expected, result2);
    }

    #[test]
    fn division() {
        let vector = Vec3::new(1.0, 2.0, 3.0);
        let divider = 2.0;
        let result = vector / divider;
        let expected = Vec3::new(0.5, 1.0, 1.5);
        assert_vec3_equal!(expected, result);
    }

    #[test]
    fn length() {
        let vector = Vec3::new(1.0, 2.0, 3.0);
        let result = vector.length();
        let expected = 14.0_f64.sqrt();
        assert_eq!(expected, result);
    }

    #[test]
    fn unit_vector() {
        let vector = Vec3::new(1.0, 2.0, 3.0);
        let result = vector.unit_vector();
        let expected = vector / 14.0_f64.sqrt();
        assert_vec3_equal!(expected, result);
    }

    #[test]
    fn dot() {
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        let vector2 = Vec3::new(4.0, 5.0, 6.0);
        let result = vector1.dot(vector2);
        let expected = 32.0;
        assert_eq!(expected, result);
    }
}
