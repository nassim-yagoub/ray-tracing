use crate::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn write_color(&self) {
        println!(
            "{} {} {}",
            (255.99 * self.x) as u8,
            (255.99 * self.y) as u8,
            (255.99 * self.z) as u8
        );
    }
}
