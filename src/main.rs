use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::vec3::Point3;

mod camera;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod vec3;

// Image constants
const ASPECT_RATIO: f64 = 16. / 9.;
const IMAGE_WIDTH: u32 = 400;

fn main() {
    // World
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));

    let camera = Camera::new(ASPECT_RATIO, IMAGE_WIDTH);

    camera.render(&world);
}
