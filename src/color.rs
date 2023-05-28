use crate::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn write_color(&self, samples_per_pixel: i32) {
        let scale = 1.0 / samples_per_pixel as f64;
        println!(
            "{} {} {}",
            (256.0 * Self::clamp(self.x * scale, 0.0, 0.999)) as u8,
            (256.0 * Self::clamp(self.y * scale, 0.0, 0.999)) as u8,
            (256.0 * Self::clamp(self.z * scale, 0.0, 0.999)) as u8
        );
    }

    fn clamp(x: f64, min: f64, max: f64) -> f64 {
        if x < min {
            return min;
        };
        if x > max {
            return max;
        };
        return x;
    }
}
