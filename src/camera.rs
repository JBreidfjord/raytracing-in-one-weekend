use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use rand::Rng;

use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Color, Point3, Vec3};

pub struct Camera {
    aspect_ratio: f64,        // Ratio of image width over height
    image_width: u32,         // Rendered image width in pixel count
    samples_per_pixel: u32,   // Count of random samples for each pixel
    max_depth: u32,           // Maximum number of ray bounces in scene
    image_height: u32,        // Rendered image height
    pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples
    center: Point3,           // Camera center
    pixel00_loc: Point3,      // Location of pixel (0, 0)
    pixel_delta_u: Vec3,      // Offset to pixel to the right
    pixel_delta_v: Vec3,      // Offset to pixel below
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
    ) -> Self {
        let mut cam = Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            image_height: 1,
            pixel_samples_scale: 1.0,
            center: Point3::default(),
            pixel00_loc: Point3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
        };
        cam.initialize();

        cam
    }

    pub fn render(&self, world: &impl Hittable) {
        let bar = ProgressBar::new(self.image_height.into());
        let mut buffer: RgbImage = ImageBuffer::new(self.image_width, self.image_height);
        let mut prev_y = self.image_height + 1; // Start at a value outside the image
        for (x, y, pixel) in buffer.enumerate_pixels_mut() {
            // Increment the progress bar on new scan lines
            if y != prev_y {
                bar.inc(1);
                prev_y = y;
            }

            let mut pixel_color = Color::new(0., 0., 0.);
            for _ in 0..self.samples_per_pixel {
                let ray = self.get_ray(x.into(), y.into());
                pixel_color += Self::ray_color(&ray, self.max_depth, world);
            }

            pixel_color *= self.pixel_samples_scale;
            *pixel = pixel_color.into();
        }

        bar.finish();
        buffer
            .save("image.png")
            .expect("Failed to save buffer to image");
    }

    fn initialize(&mut self) {
        self.image_height =
            Self::calculate_image_height(self.image_width.into(), self.aspect_ratio);

        self.pixel_samples_scale = 1. / f64::from(self.samples_per_pixel);

        self.center = Point3::new(0., 0., 0.);

        // Determine viewport dimensions
        let focal_length = 1.;
        let viewport_height = 2.;
        let viewport_width =
            viewport_height * f64::from(self.image_width) / f64::from(self.image_height);

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = Vec3::new(viewport_width, 0., 0.);
        let viewport_v = Vec3::new(0., -viewport_height, 0.);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = &viewport_u / self.image_width.into();
        self.pixel_delta_v = &viewport_v / self.image_height.into();

        // Calculate the location of the upper left pixel
        let viewport_upper_left =
            &self.center - Vec3::new(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
        self.pixel00_loc = viewport_upper_left + 0.5 * (&self.pixel_delta_u + &self.pixel_delta_v);
    }

    fn ray_color(ray: &Ray, depth: u32, world: &impl Hittable) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered
        if depth == 0 {
            return Color::new(0., 0., 0.);
        }

        if let Some(hit_record) = world.hit(ray, &Interval::new(0.001, f64::INFINITY)) {
            let direction = hit_record.normal() + Vec3::random_unit_vector();
            return 0.5
                * Self::ray_color(
                    &Ray::new(hit_record.p().clone(), direction),
                    depth - 1,
                    world,
                );
        }

        let unit_direction = ray.direction().unit();
        let a = 0.5 * (unit_direction.y() + 1.0);
        let white = Color::new(1., 1., 1.);
        let blue = Color::new(0.5, 0.7, 1.0);
        (1. - a) * white + a * blue
    }

    /// Construct a camera ray originating from the origin and directed at
    /// a randomly sampled point around the pixel location (i, j)
    fn get_ray(&self, i: f64, j: f64) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = &self.pixel00_loc
            + (&self.pixel_delta_u * (i + offset.x()))
            + (&self.pixel_delta_v * (j + offset.y()));

        let ray_origin = &self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin.clone(), ray_direction)
    }

    /// Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square
    fn sample_square() -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3::new(rng.gen::<f64>() - 0.5, rng.gen::<f64>() - 0.5, 0.)
    }

    fn calculate_image_height(image_width: f64, aspect_ratio: f64) -> u32 {
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let image_height = (image_width / aspect_ratio) as u32;
        if image_height < 1 {
            1
        } else {
            image_height
        }
    }
}
