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
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(&Color, Ray)> {
        let reflected = ray.direction().reflect(hit_record.normal());
        let reflected = reflected.unit() + (self.fuzz * Vec3::random_unit_vector());
        let scattered = Ray::new(hit_record.p().clone(), reflected);
        let attenuation = &self.albedo;
        if scattered.direction().dot(hit_record.normal()) > 0. {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    /// Refractive index in vacuum or air, or the ratio of the material's refractive index over
    /// the refractive index of the enclosing media
    refraction_index: f64,
    color: Color,
}

impl Dielectric {
    pub const fn new(refraction_index: f64) -> Self {
        Self {
            refraction_index,
            color: Color::new(1., 1., 1.),
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(&Color, Ray)> {
        let refraction_ratio = if hit_record.front_face() {
            1. / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray.direction().unit();

        let cos_theta = (-&unit_direction).dot(hit_record.normal()).min(1.0);
        let sin_theta = cos_theta.mul_add(-cos_theta, 1.0).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract {
            unit_direction.reflect(hit_record.normal())
        } else {
            unit_direction.refract(hit_record.normal(), refraction_ratio)
        };

        let scattered = Ray::new(hit_record.p().clone(), direction);
        Some((&self.color, scattered))
    }
}
