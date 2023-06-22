use rand::Rng;

use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit_record: HitRecord) -> (Color, Ray, bool);
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, hit_record: HitRecord) -> (Color, Ray, bool) {
        let mut scatter_direction: Vec3 = hit_record.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        let scattered: Ray = Ray::new(hit_record.p, scatter_direction);

        return (self.albedo, scattered, true);
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit_record: HitRecord) -> (Color, Ray, bool) {
        let reflected: Vec3 = r_in.direction.unit_vector().reflect(hit_record.normal);
        let scattered: Ray = Ray::new(
            hit_record.p,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        );
        let success: bool = scattered.direction.dot(hit_record.normal) > 0.0;

        return (self.albedo, scattered, success);
    }
}

pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Dielectric {
        Dielectric { ir }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0: f64 = (1.0 - ref_idx) / (1.0 + ref_idx);

        return r0 * r0 + (1.0 - r0 * r0) * (1.0 - cosine).powf(5.0);
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit_record: HitRecord) -> (Color, Ray, bool) {
        let refraction_ratio: f64 = if hit_record.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction: Vec3 = r_in.direction.unit_vector();

        let cos_theta: f64 = (-unit_direction.dot(hit_record.normal)).min(1.0);
        let sin_theta: f64 = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract: bool = refraction_ratio * sin_theta > 1.0;

        let direction: Vec3;
        let mut rng = rand::thread_rng();

        if cannot_refract
            || Self::reflectance(cos_theta, refraction_ratio) > rng.gen_range(0.0..1.0)
        {
            direction = unit_direction.reflect(hit_record.normal)
        } else {
            direction = unit_direction.refract(hit_record.normal, refraction_ratio)
        }

        return (
            Color::new(1.0, 1.0, 1.0),
            Ray::new(hit_record.p, direction),
            true,
        );
    }
}
