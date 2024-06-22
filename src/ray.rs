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

    pub(crate) fn at(&self, t: f64) -> Point3 {
        self.origin() + self.direction() * t
    }

    pub(crate) fn color(&self) -> Color {
        let t = hit_sphere(&Point3::new(0., 0., -1.), 0.5, self);
        if t > 0.0 {
            let n = (self.at(t) - Vec3::new(0., 0., -1.)).unit();
            return 0.5 * Color::new(n.x() + 1., n.y() + 1., n.z() + 1.);
        }

        let unit_direction = self.direction().unit();
        let a = 0.5 * (unit_direction.y() + 1.0);
        let white = Color::new(1., 1., 1.);
        let blue = Color::new(0.5, 0.7, 1.0);
        (1. - a) * white + a * blue
    }
}

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = center - ray.origin();
    let a = ray.direction().length_squared();
    let h = ray.direction().dot(&oc);
    let c = oc.length_squared() - radius.powi(2);
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (h - discriminant.sqrt()) / a
    }
}
