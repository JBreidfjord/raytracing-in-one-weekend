use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

use crate::vec3::Color;

mod ray;
mod vec3;

fn main() {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    let bar = ProgressBar::new(IMAGE_HEIGHT as u64);
    let mut buffer: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    let mut prev_y = IMAGE_HEIGHT + 1; // Start at a value outside the image
    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        if y != prev_y {
            bar.inc(1);
            prev_y = y
        }

        let r = x as f64 / (IMAGE_WIDTH - 1) as f64;
        let g = y as f64 / (IMAGE_HEIGHT - 1) as f64;
        let pixel_color = Color::new(r, g, 0.0);

        *pixel = pixel_color.into();
    }

    bar.finish();
    buffer.save("image.png").unwrap();
}
