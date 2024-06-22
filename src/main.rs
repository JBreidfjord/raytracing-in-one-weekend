#![feature(const_fn_floating_point_arithmetic)]

use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Point3, Vec3};

mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod vec3;

// Image constants
const ASPECT_RATIO: f64 = 16. / 9.;
const IMAGE_WIDTH: u32 = 400;
// Calculate the image height, and ensure that it's at least 1
const IMAGE_HEIGHT: u32 = calculate_image_height(IMAGE_WIDTH as f64, ASPECT_RATIO);

// Camera constants
const FOCAL_LENGTH: f64 = 1.;
const VIEWPORT_HEIGHT: f64 = 2.;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64;

const fn calculate_image_height(image_width: f64, aspect_ratio: f64) -> u32 {
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    let image_height = (image_width / aspect_ratio) as u32;
    if image_height < 1 {
        1
    } else {
        image_height
    }
}

fn main() {
    // World
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));

    // Camera
    let camera_center = Point3::new(0., 0., 0.);

    // Calculate the vectors across the horizontal and down the vertical viewport edges
    let viewport_u = Vec3::new(VIEWPORT_WIDTH, 0., 0.);
    let viewport_v = Vec3::new(0., -VIEWPORT_HEIGHT, 0.);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = &viewport_u / IMAGE_WIDTH.into();
    let pixel_delta_v = &viewport_v / IMAGE_HEIGHT.into();

    // Calculate the location of the upper left pixel
    let viewport_upper_left =
        &camera_center - Vec3::new(0., 0., FOCAL_LENGTH) - viewport_u / 2. - viewport_v / 2.;
    let pixel00_loc = viewport_upper_left + 0.5 * (&pixel_delta_u + &pixel_delta_v);

    // Render
    let bar = ProgressBar::new(IMAGE_HEIGHT.into());
    let mut buffer: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    let mut prev_y = IMAGE_HEIGHT + 1; // Start at a value outside the image
    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        // Increment the progress bar on new scan lines
        if y != prev_y {
            bar.inc(1);
            prev_y = y;
        }

        let pixel_center = &pixel00_loc + (&pixel_delta_u * x.into()) + (&pixel_delta_v * y.into());
        let ray_direction = pixel_center - &camera_center;
        let ray = Ray::new(camera_center.clone(), ray_direction);

        let pixel_color = ray.color(&world);
        *pixel = pixel_color.into();
    }

    bar.finish();
    buffer
        .save("image.png")
        .expect("Failed to save buffer to image");
}
