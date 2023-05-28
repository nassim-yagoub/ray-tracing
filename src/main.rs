mod camera;
mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use color::Color;
use hittable::Hittable;
use hittable_list::HittableList;
use rand::Rng;
use ray::Ray;
use sphere::Sphere;
use vec3::Point3;

fn ray_color(ray: &Ray, world: &HittableList) -> Color {
    let rec = world.hit(ray, f64::INFINITY, 0.0);

    match rec {
        Some(record) => {
            return 0.5
                * Color::new(
                    record.normal.x + 1.0,
                    record.normal.y + 1.0,
                    record.normal.z + 1.0,
                );
        }
        None => {
            let unit_direction = ray.direction.unit_vector();
            let t = 0.5 * (unit_direction.y + 1.0);

            return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
        }
    }
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 10;

    // World
    let mut world = HittableList::new();
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    // Camera
    let camera = Camera::new();

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let mut rng = rand::thread_rng();

    for i in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rLines remaining: {}", i);
        for j in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for s in 0..SAMPLES_PER_PIXEL {
                let v = (rng.gen::<f64>() + i as f64) / (IMAGE_HEIGHT - 1) as f64;
                let u = (rng.gen::<f64>() + j as f64) / (IMAGE_WIDTH - 1) as f64;

                let r = camera.get_ray(u, v);

                pixel_color = pixel_color + ray_color(&r, &world);
            }

            pixel_color.write_color(SAMPLES_PER_PIXEL)
        }
    }
}
