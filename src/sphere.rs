use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub(crate) struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub(crate) fn new(center: Point3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }

    fn center(&self) -> &Point3 {
        &self.center
    }

    fn radius(&self) -> &f64 {
        &self.radius
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = self.center() - ray.origin();
        let a = ray.direction().length_squared();
        let h = ray.direction().dot(&oc);
        let c = oc.length_squared() - self.radius().powi(2);

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let root = (h - sqrt_d) / a;
        if root <= t_min || t_max <= root {
            let root = (h + sqrt_d) / a;
            if root <= t_min || t_max <= root {
                return None;
            }
        }

        let t = root;
        let p = ray.at(t);
        let outward_normal = (&p - self.center()) / *self.radius();
        let mut hit_record = HitRecord::new(p, Vec3::new(0., 0., 0.), t);
        hit_record.set_face_normal(ray, outward_normal);

        Some(hit_record)
    }
}
