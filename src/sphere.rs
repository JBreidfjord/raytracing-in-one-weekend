use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Material,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Material) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
            mat,
        }
    }

    const fn center(&self) -> &Point3 {
        &self.center
    }

    const fn radius(&self) -> &f64 {
        &self.radius
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let oc = self.center() - ray.origin();
        let a = ray.direction().length_squared();
        let half_b = ray.direction().dot(&oc);
        let c = oc.length_squared() - self.radius().powi(2);

        let discriminant = a.mul_add(-c, half_b.powi(2));
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (half_b - sqrt_d) / a;
        if !ray_t.surrounds(root) {
            root = (half_b + sqrt_d) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let t = root;
        let p = ray.at(t);
        let outward_normal = (&p - self.center()) / *self.radius();
        let mut hit_record = HitRecord::new(p, Vec3::new(0., 0., 0.), self.mat.clone(), t);
        hit_record.set_face_normal(ray, outward_normal);

        Some(hit_record)
    }
}
