use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(&Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub const fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(&Color, Ray)> {
        let mut scatter_direction = hit_record.normal() + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal().clone();
        }

        let scattered = Ray::new(hit_record.p().clone(), scatter_direction);
        let attenuation = &self.albedo;

        Some((attenuation, scattered))
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub const fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(&Color, Ray)> {
        let reflected = ray.direction().reflect(hit_record.normal());
        let scattered = Ray::new(hit_record.p().clone(), reflected);
        let attenuation = &self.albedo;
        Some((attenuation, scattered))
    }
}
