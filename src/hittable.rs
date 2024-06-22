use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub(crate) struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
}

impl HitRecord {
    pub(crate) fn new(p: Point3, normal: Vec3, t: f64) -> HitRecord {
        HitRecord { p, normal, t }
    }
}

pub(crate) trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
