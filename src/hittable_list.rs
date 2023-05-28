use crate::{
    hittable::{HitRecord, Hittable},
    sphere::Sphere,
    vec3::{Point3, Vec3},
};

pub struct HittableList {
    pub objects: Vec<Sphere>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }
    pub fn add(&mut self, hittable: Sphere) {
        self.objects.push(hittable);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &crate::ray::Ray, t_max: f64, t_min: f64) -> Option<HitRecord> {
        let mut temp_rec = HitRecord::new(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            t_max,
            true,
        );

        for object in &self.objects {
            let hit_value = object.hit(ray, t_max, t_min);
            match hit_value {
                Some(record) => {
                    if record.t < temp_rec.t {
                        temp_rec = record;
                    }
                }
                None => {}
            }
        }

        if temp_rec.t == t_max {
            None // No hit
        } else {
            Some(temp_rec)
        }
    }
}
