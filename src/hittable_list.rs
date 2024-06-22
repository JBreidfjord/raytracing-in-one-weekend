use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub(crate) struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub(crate) fn new(object: Box<dyn Hittable>) -> HittableList {
        HittableList {
            objects: vec![object],
        }
    }

    pub(crate) fn clear(&mut self) {
        self.objects.clear()
    }

    pub(crate) fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object)
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut maybe_record: Option<HitRecord> = None;
        for object in self.objects.iter() {
            if let Some(hit_record) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = *hit_record.t();
                maybe_record = Some(hit_record);
            }
        }

        maybe_record
    }
}
