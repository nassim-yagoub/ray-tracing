mod color;
use color::Color;

fn main() {
    const IMAGE_HEIGHT: i32 = 256;
    const IMAGE_WIDTH: i32 = 256;

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for i in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rLines remaining: {}", i);
        for j in 0..IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.25;

            let pixel_color: Color = Color::new(r, g, b);

            pixel_color.write_color()
        }
    }
}
