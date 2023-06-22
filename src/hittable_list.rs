use crate::hittable::{HitRecord, Hittable};

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }
    pub fn add(&mut self, hittable: impl Hittable + 'static) {
        self.objects.push(Box::new(hittable));
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &crate::ray::Ray, t_max: f64, t_min: f64) -> Option<HitRecord> {
        let mut temp_rec: Option<HitRecord> = None;
        let mut closest = f64::MAX;

        for object in &self.objects {
            let hit_value = object.hit(ray, t_max, t_min);
            match hit_value {
                Some(record) => {
                    if record.t < closest {
                        closest = record.t;
                        temp_rec = Some(record);
                    }
                }
                None => {}
            }
        }

        temp_rec
    }
}
