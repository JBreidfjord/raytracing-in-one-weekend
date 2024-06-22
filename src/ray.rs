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
        Color::new(0., 0., 0.)
    }
}
