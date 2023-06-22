use crate::{
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        p: Point3,
        normal: Vec3,
        t: f64,
        front_face: bool,
        material: &'a dyn Material,
    ) -> HitRecord {
        HitRecord {
            p,
            normal: if front_face { normal } else { -normal },
            t,
            front_face,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_max: f64, t_min: f64) -> Option<HitRecord>;
}
