use std::rc::Rc;

use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    mat: Rc<dyn Material>,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub const fn new(p: Point3, normal: Vec3, mat: Rc<dyn Material>, t: f64) -> Self {
        Self {
            p,
            normal,
            mat,
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

        self.front_face = Self::is_front_face(ray, &outward_normal);
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }

    pub const fn normal(&self) -> &Vec3 {
        &self.normal
    }

    pub const fn t(&self) -> f64 {
        self.t
    }

    pub const fn p(&self) -> &Point3 {
        &self.p
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord>;
}
