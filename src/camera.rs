use image::{ImageBuffer, RgbImage};
use indicatif::ParallelProgressIterator;
use rand::Rng;
use rayon::prelude::*;

use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Color, Point3, Vec3};

pub struct Camera {
    image_width: u32,       // Rendered image width in pixel count
    samples_per_pixel: u32, // Count of random samples for each pixel
    max_depth: u32,         // Maximum number of ray bounces in scene

    defocus_angle: f64, // Variation angle of rays through each pixel

    image_height: u32,        // Rendered image height
    pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples
    center: Point3,           // Camera center
    pixel00_loc: Point3,      // Location of pixel (0, 0)
    pixel_delta_u: Vec3,      // Offset to pixel to the right
    pixel_delta_v: Vec3,      // Offset to pixel below
    defocus_disk_u: Vec3,     // Defocus disk horizontal radius
    defocus_disk_v: Vec3,     // Defocus disk vertical radius
}

impl Camera {
    pub const fn builder() -> CamBuilder {
        CamBuilder::default()
    }

    pub fn render(&self, world: &HittableList) {
        let mut buffer: RgbImage = ImageBuffer::new(self.image_width, self.image_height);
        let length = buffer.len() as u64;
        buffer
            .par_enumerate_pixels_mut()
            .into_par_iter()
            .progress_count(length)
            .for_each(|(x, y, pixel)| {
                let mut pixel_color = Color::new(0., 0., 0.);
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(x.into(), y.into());
                    pixel_color += Self::ray_color(&ray, self.max_depth, world);
                }

                pixel_color *= self.pixel_samples_scale;
                *pixel = pixel_color.into();
            });

        buffer
            .save("image.png")
            .expect("Failed to save buffer to image");
    }

    fn ray_color(ray: &Ray, depth: u32, world: &dyn Hittable) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered
        if depth == 0 {
            return Color::new(0., 0., 0.);
        }

        if let Some(hit_record) = world.hit(ray, &Interval::new(0.001, f64::INFINITY)) {
            if let Some((attenuation, scattered)) = hit_record.material().scatter(ray, &hit_record)
            {
                return attenuation * Self::ray_color(&scattered, depth - 1, world);
            }
            return Color::new(0., 0., 0.);
        }

        let unit_direction = ray.direction().unit();
        let a = 0.5 * (unit_direction.y() + 1.0);
        let white = Color::new(1., 1., 1.);
        let blue = Color::new(0.5, 0.7, 1.0);
        (1. - a) * white + a * blue
    }

    /// Construct a camera ray originating from the defocus disk and directed at
    /// a randomly sampled point around the pixel location (i, j)
    fn get_ray(&self, i: f64, j: f64) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = &self.pixel00_loc
            + (&self.pixel_delta_u * (i + offset.x()))
            + (&self.pixel_delta_v * (j + offset.y()));

        let ray_origin = if self.defocus_angle <= 0. {
            &self.center
        } else {
            &self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin.clone(), ray_direction)
    }

    /// Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square
    fn sample_square() -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3::new(rng.gen::<f64>() - 0.5, rng.gen::<f64>() - 0.5, 0.)
    }

    /// Returns a random point in the camera defocus disk
    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        &self.center + (p[0] * &self.defocus_disk_u) + (p[1] * &self.defocus_disk_v)
    }
}

pub struct CamBuilder {
    aspect_ratio: f64,      // Ratio of image width over height
    image_width: u32,       // Rendered image width in pixel count
    samples_per_pixel: u32, // Count of random samples for each pixel
    max_depth: u32,         // Maximum number of ray bounces in scene

    vfov: f64,         // Vertical view angle (field of view)
    look_from: Point3, // Point camera is looking from
    look_at: Point3,   // Point camera is looking at
    vup: Vec3,         // Camera-relative "up" direction

    defocus_angle: f64, // Variation angle of rays through each pixel
    focus_dist: f64,    // Distance from camera look_from point to plane of perfect focus
}

impl CamBuilder {
    pub const fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.,
            look_from: Point3::new(0., 0., 0.),
            look_at: Point3::new(0., 0., -1.),
            vup: Vec3::new(0., 1., 0.),
            defocus_angle: 0.,
            focus_dist: 10.,
        }
    }

    pub fn build(self) -> Camera {
        let image_height = Self::calculate_image_height(self.image_width.into(), self.aspect_ratio);

        let pixel_samples_scale = 1. / f64::from(self.samples_per_pixel);

        let center = self.look_from.clone();

        // Determine viewport dimensions
        let theta = self.vfov.to_radians();
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h * self.focus_dist;
        let viewport_width =
            viewport_height * f64::from(self.image_width) / f64::from(image_height);

        let w = (&self.look_from - &self.look_at).unit();
        let u = self.vup.cross(&w).unit();
        let v = w.cross(&u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = viewport_width * &u;
        let viewport_v = viewport_height * -&v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = &viewport_u / self.image_width.into();
        let pixel_delta_v = &viewport_v / image_height.into();

        // Calculate the location of the upper left pixel
        let viewport_upper_left =
            &center - (self.focus_dist * &w) - viewport_u / 2. - viewport_v / 2.;
        let pixel00_loc = viewport_upper_left + 0.5 * (&pixel_delta_u + &pixel_delta_v);

        // Calculate the camera defocus disk basis vectors
        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.).to_radians().tan();
        let defocus_disk_u = &u * defocus_radius;
        let defocus_disk_v = &v * defocus_radius;

        Camera {
            image_width: self.image_width,
            samples_per_pixel: self.samples_per_pixel,
            max_depth: self.max_depth,
            defocus_angle: self.defocus_angle,
            image_height,
            pixel_samples_scale,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_disk_u,
            defocus_disk_v,
        }
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

    pub const fn aspect_ratio(mut self, aspect_ratio: f64) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub const fn image_width(mut self, image_width: u32) -> Self {
        self.image_width = image_width;
        self
    }

    pub const fn samples_per_pixel(mut self, samples_per_pixel: u32) -> Self {
        self.samples_per_pixel = samples_per_pixel;
        self
    }

    pub const fn max_depth(mut self, max_depth: u32) -> Self {
        self.max_depth = max_depth;
        self
    }

    pub const fn vfov(mut self, vfov: f64) -> Self {
        self.vfov = vfov;
        self
    }

    pub const fn look_from(mut self, look_from: Point3) -> Self {
        self.look_from = look_from;
        self
    }

    pub const fn look_at(mut self, look_at: Point3) -> Self {
        self.look_at = look_at;
        self
    }

    pub const fn vup(mut self, vup: Vec3) -> Self {
        self.vup = vup;
        self
    }

    pub const fn defocus_angle(mut self, defocus_angle: f64) -> Self {
        self.defocus_angle = defocus_angle;
        self
    }

    pub const fn focus_dist(mut self, focus_dist: f64) -> Self {
        self.focus_dist = focus_dist;
        self
    }
}
