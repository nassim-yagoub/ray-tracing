use show_image::{create_window, ImageInfo, ImageView};
use std::{thread, time};

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let image_height: u8 = 255;
    let image_width: u8 = 255;

    //let pixels = 3 * image_width * image_height;

    let mut pixel_data: [u8; 3 * 255 * 255] = [0; 3 * 255 * 255];

    for i in 0..255 {
        for j in 0..255 {
            pixel_data[255 * 3 * i + 3 * j] = j as u8;
            pixel_data[255 * 3 * i + 3 * j + 1] = i as u8;
            pixel_data[255 * 3 * i + 3 * j + 2] = 64;
        }
    }

    let image = ImageView::new(
        ImageInfo::rgb8(image_width as u32, image_height as u32),
        &pixel_data,
    );

    // Create a window with default options and display the image.
    let window = create_window("image", Default::default())?;
    window.set_image("image-002", image)?;

    let ten_millis = time::Duration::from_millis(10000);
    thread::sleep(ten_millis);

    Ok(())
}
