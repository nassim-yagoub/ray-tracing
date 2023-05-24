pub struct Color {
    x: f64,
    y: f64,
    z: f64,
}

impl Color {
    pub fn new(x: f64, y: f64, z: f64) -> Color {
        Color { x, y, z }
    }

    pub fn write_color(&self) {
        println!(
            "{} {} {}",
            (255.99 * self.x) as u8,
            (255.99 * self.y) as u8,
            (255.99 * self.z) as u8
        );
    }
}
