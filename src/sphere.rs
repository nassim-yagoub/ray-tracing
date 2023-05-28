use crate::{
    hittable::{HitRecord, Hittable},
    vec3::{Point3, Vec3},
};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, t_max: f64, t_min: f64) -> Option<HitRecord> {
        let oc: Vec3 = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Try small root then big one if the small is off limits
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let outward_normal = (ray.at(root) - self.center) / self.radius;
        let front_face = ray.direction.dot(outward_normal) < 0.0;

        let rec = HitRecord::new(ray.at(root), outward_normal, root, front_face);

        return Some(rec);
    }
}

#[cfg(test)]
mod tests {
    use crate::ray::Ray;

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

    macro_rules! assert_hit_record_equal {
        ($expected:expr, $actual:expr) => {
            let tolerance = 0.0001;
            assert_relative_eq!($expected.t, $actual.t, epsilon = tolerance);
            assert_vec3_equal!($expected.p, $actual.p);
            assert_vec3_equal!($expected.normal, $actual.normal);
            assert!($expected.front_face == $actual.front_face)
        };
    }

    #[test]
    fn hit_first_point() {
        let origin = Point3::new(0.0, 0.0, 0.0);
        let direction = Vec3::new(0.0, 0.0, -1.0);
        let ray = Ray::new(origin, direction);

        let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);

        let result = sphere.hit(&ray, 3.0, 0.0);
        let expected = HitRecord::new(
            Point3::new(0.0, 0.0, -0.5),
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            0.5,
            true,
        );

        match result {
            Some(record) => {
                assert_hit_record_equal!(expected, record);
            }
            None => {
                panic!("Should have hit")
            }
        }
    }

    #[test]
    fn hit_second_point() {
        let origin = Point3::new(0.0, 0.0, 0.0);
        let direction = Vec3::new(0.0, 0.0, -1.0);
        let ray = Ray::new(origin, direction);

        let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);

        let result = sphere.hit(&ray, 3.0, 1.0);
        let expected = HitRecord::new(
            Point3::new(0.0, 0.0, -1.5),
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            1.5,
            false,
        );

        match result {
            Some(record) => {
                assert_hit_record_equal!(expected, record);
            }
            None => {
                panic!("Should have hit")
            }
        }
    }

    #[test]
    fn hit_none_range() {
        let origin = Point3::new(0.0, 0.0, 0.0);
        let direction = Vec3::new(0.0, 0.0, -1.0);
        let ray = Ray::new(origin, direction);

        let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);

        let result = sphere.hit(&ray, 0.4, 0.0);

        assert!(result.is_none());
    }

    #[test]
    fn hit_none_direction() {
        let origin = Point3::new(0.0, 0.0, 0.0);
        let direction = Vec3::new(0.0, 1.0, 0.0);
        let ray = Ray::new(origin, direction);

        let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);

        let result = sphere.hit(&ray, 0.4, 0.0);

        assert!(result.is_none());
    }
}
