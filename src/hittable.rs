use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub(crate) struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub(crate) fn new(p: Point3, normal: Vec3, t: f64) -> HitRecord {
        HitRecord {
            p,
            normal,
            t,
            front_face: false,
        }
    }

    fn is_front_face(ray: &Ray, outward_normal: &Vec3) -> bool {
        ray.direction().dot(outward_normal) < 0.0
    }

    pub(crate) fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.
        // TODO: Can we enforce this with the type system?

        self.front_face = HitRecord::is_front_face(ray, &outward_normal);
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }

    pub(crate) fn normal(&self) -> &Vec3 {
        &self.normal
    }

    pub(crate) fn t(&self) -> &f64 {
        &self.t
    }
}

pub(crate) trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
