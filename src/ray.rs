use crate::vec3::{Color, Point3, Vec3};

pub(crate) struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub(crate) fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
        }
    }

    pub(crate) fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub(crate) fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub(crate) fn at(self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }

    pub(crate) fn color(&self) -> Color {
        if hit_sphere(&Point3::new(0., 0., -1.), 0.5, self) {
            return Color::new(1., 0., 0.);
        }

        let unit_direction = self.direction().unit();
        let a = 0.5 * (unit_direction.y() + 1.0);
        let white = Color::new(1., 1., 1.);
        let blue = Color::new(0.5, 0.7, 1.0);
        (1. - a) * white + a * blue
    }
}

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> bool {
    let oc = center - ray.origin();
    let a = ray.direction().dot(ray.direction());
    let b = -2. * ray.direction().dot(&oc);
    let c = oc.dot(&oc) - radius.powi(2);
    let discriminant = b * b - 4. * a * c;
    // Discriminant will be negative if there are no real solutions,
    // indicating that the ray has not hit the sphere
    discriminant >= 0.0
}
