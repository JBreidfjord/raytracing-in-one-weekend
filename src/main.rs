use indicatif::ProgressBar;

use crate::vec3::Color;

mod vec3;

fn main() {
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = 256;

    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    let bar = ProgressBar::new(IMAGE_HEIGHT);
    for j in 0..IMAGE_HEIGHT {
        bar.inc(1);
        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let pixel_color = Color::new(r, g, 0.0);
            println!("{}", pixel_color.format_color());
        }
    }
    bar.finish();
}
