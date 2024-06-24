use rand::{Rng, thread_rng};

use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

#[derive(Clone)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
    Dielectric { refraction_index: f64, color: Color },
}

impl Material {
    pub fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(&Color, Ray)> {
        match self {
            Self::Lambertian { albedo } => {
                let mut scatter_direction = hit_record.normal() + Vec3::random_unit_vector();

                // Catch degenerate scatter direction
                if scatter_direction.near_zero() {
                    scatter_direction = hit_record.normal().clone();
                }

                let scattered = Ray::new(hit_record.p().clone(), scatter_direction);
                let attenuation = albedo;

                Some((attenuation, scattered))
            }

            Self::Metal { albedo, fuzz } => {
                let reflected = ray.direction().reflect(hit_record.normal());
                let reflected = reflected.unit() + (*fuzz * Vec3::random_unit_vector());
                let scattered = Ray::new(hit_record.p().clone(), reflected);
                let attenuation = albedo;
                if scattered.direction().dot(hit_record.normal()) > 0. {
                    Some((attenuation, scattered))
                } else {
                    None
                }
            }

            Self::Dielectric {
                refraction_index,
                color,
            } => {
                let refraction_ratio = if hit_record.front_face() {
                    1. / refraction_index
                } else {
                    *refraction_index
                };

                let unit_direction = ray.direction().unit();

                let cos_theta = (-&unit_direction).dot(hit_record.normal()).min(1.0);
                let sin_theta = cos_theta.mul_add(-cos_theta, 1.0).sqrt();

                let mut rng = thread_rng();
                let cannot_refract = refraction_ratio * sin_theta > 1.0;
                let direction =
                    if cannot_refract || reflectance(cos_theta, refraction_ratio) > rng.gen() {
                        unit_direction.reflect(hit_record.normal())
                    } else {
                        unit_direction.refract(hit_record.normal(), refraction_ratio)
                    };

                let scattered = Ray::new(hit_record.p().clone(), direction);
                Some((color, scattered))
            }
        }
    }
}

/// Schlick's approximation for reflectance
fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = (1. - refraction_index) / (1. + refraction_index);
    let r0 = r0.powi(2);
    (1. - r0).mul_add((1. - cosine).powi(5), r0)
}
