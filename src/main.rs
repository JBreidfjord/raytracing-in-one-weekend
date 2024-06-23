use std::rc::Rc;

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::material::Lambertian;
use crate::sphere::Sphere;
use crate::vec3::{Color, Point3};

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod vec3;

// Image constants
const ASPECT_RATIO: f64 = 16. / 9.;
const IMAGE_WIDTH: u32 = 400;
const SAMPLES_PER_PIXEL: u32 = 100;
const MAX_DEPTH: u32 = 50;

fn main() {
    // Materials
    let grey_diffuse = Rc::new(Lambertian::new(Color::new(128., 128., 128.)));

    // World
    let mut world = HittableList::default();
    world.add(Rc::new(Sphere::new(
        Point3::new(0., 0., -1.),
        0.5,
        grey_diffuse.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        grey_diffuse,
    )));

    let camera = Camera::new(ASPECT_RATIO, IMAGE_WIDTH, SAMPLES_PER_PIXEL, MAX_DEPTH);

    camera.render(&world);
}
