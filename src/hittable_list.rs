use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn default() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.max();
        let mut maybe_record: Option<HitRecord> = None;
        for object in &self.objects {
            if let Some(hit_record) = object.hit(ray, &Interval::new(ray_t.min(), closest_so_far)) {
                closest_so_far = hit_record.t();
                maybe_record = Some(hit_record);
            }
        }

        maybe_record
    }
}
