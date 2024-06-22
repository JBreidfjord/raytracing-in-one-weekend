use crate::hittable::Hittable;
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

    pub(crate) fn color(&self, world: &impl Hittable) -> Color {
        if let Some(hit_record) = world.hit(self, 0.0, f64::INFINITY) {
            return 0.5 * (hit_record.normal() + Color::new(1., 1., 1.));
        }

        let unit_direction = self.direction().unit();
        let a = 0.5 * (unit_direction.y() + 1.0);
        let white = Color::new(1., 1., 1.);
        let blue = Color::new(0.5, 0.7, 1.0);
        (1. - a) * white + a * blue
    }
}
