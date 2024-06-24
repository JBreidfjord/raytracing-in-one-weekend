use rand::{Rng, thread_rng};

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::material::Material::{Dielectric, Lambertian, Metal};
use crate::sphere::Sphere;
use crate::vec3::{Color, Point3, Vec3};

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod vec3;

fn main() {
    let mut world = HittableList::default();

    world.add(Box::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        Lambertian {
            albedo: Color::new(0.5, 0.5, 0.5),
        },
    )));

    let mut rng = thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Point3::new(
                0.9f64.mul_add(rng.gen(), a.into()),
                0.2,
                0.9f64.mul_add(rng.gen(), b.into()),
            );
            if (&center - Point3::new(4., 0.2, 0.)).length() <= 0.9 {
                continue;
            }

            match choose_mat {
                ..0.8 => {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Lambertian { albedo };
                    world.add(Box::new(Sphere::new(center.clone(), 0.2, sphere_material)));
                }
                0.8..0.95 => {
                    // metal
                    let albedo = Color::random_in_interval(0.5, 1.);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let sphere_material = Metal { albedo, fuzz };
                    world.add(Box::new(Sphere::new(center.clone(), 0.2, sphere_material)));
                }
                _ => {
                    // glass
                    let sphere_material = Dielectric {
                        refraction_index: 1.5,
                        color: Color::new(1., 1., 1.),
                    };
                    world.add(Box::new(Sphere::new(center.clone(), 0.2, sphere_material)));
                }
            }
        }
    }

    let material_1 = Dielectric {
        refraction_index: 1.5,
        color: Color::new(1., 1., 1.),
    };
    world.add(Box::new(Sphere::new(
        Point3::new(0., 1., 0.),
        1.,
        material_1,
    )));

    let material_2 = Lambertian {
        albedo: Color::new(0.4, 0.2, 0.1),
    };
    world.add(Box::new(Sphere::new(
        Point3::new(-4., 1., 0.),
        1.,
        material_2,
    )));

    let material_3 = Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.,
    };
    world.add(Box::new(Sphere::new(
        Point3::new(4., 1., 0.),
        1.,
        material_3,
    )));

    let camera = Camera::new(
        16. / 9.,
        1200,
        500,
        50,
        20.,
        Point3::new(13., 2., 3.),
        Point3::new(0., 0., 0.),
        Vec3::new(0., 1., 0.),
        0.6,
        10.,
    );

    camera.render(&world);
}
