use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::util::{
    random_unit_vector, random_vec_in_unit_sphere, vec_near_zero, vec_reflect, vec_refract,
    vec_refract2,
};
use macaw::Vec3;

#[derive(Clone, PartialEq, Debug)]
pub enum Material {
    Metal { albedo: Vec3, fuzz: f32 },
    Matte { albedo: Vec3 },
    Glass { refractive_index: f32 },
    Light { color: Vec3 },
}

impl Default for Material {
    fn default() -> Self {
        Material::Matte {
            albedo: Vec3::default(),
        }
    }
}

pub fn scatter(
    material: &Material,
    ray_in: &Ray,
    hit_rec: &HitRecord,
    attenuation: &mut Vec3,
    scattered_ray: &mut Ray,
) -> bool {
    match material {
        &Material::Matte { albedo } => {
            //eprintln!("Matte: {:?}", ray_in);
            let mut scatter_direction = hit_rec.normal + random_unit_vector();

            if vec_near_zero(scatter_direction) {
                scatter_direction = hit_rec.normal;
            }

            *scattered_ray = Ray::new(hit_rec.point, scatter_direction);
            *attenuation = albedo;
            true
        }
        &Material::Metal { albedo, fuzz } => {
            //eprintln!("Metal: {:?}", ray_in);
            let reflected_direction = vec_reflect(&ray_in.direction().normalize(), &hit_rec.normal);
            *scattered_ray = Ray::new(
                hit_rec.point,
                reflected_direction + fuzz * random_vec_in_unit_sphere(),
            );
            *attenuation = albedo;
            let scattered_direction = scattered_ray.direction();
            scattered_direction.dot(hit_rec.normal) > 0.0
        }
        &Material::Glass { refractive_index } => {
            *attenuation = Vec3::new(1.0, 1.0, 1.0);
            let refraction_quotion;

            if ray_in.direction().dot(hit_rec.normal) > 0.0 {
                // Change
                refraction_quotion = 1.0 / refractive_index;
            } else {
                refraction_quotion = refractive_index;
            }

            let direction_normalized = ray_in.direction().normalize();
            let refracted_direction =
                vec_refract2(&direction_normalized, &hit_rec.normal, refraction_quotion);

            *scattered_ray = Ray::new(hit_rec.point, refracted_direction);
            true
        }
        &Material::Light { color } => false,
    }
}

pub fn emitt(material: &Material, u: f32, v: f32, point: Vec3) -> Vec3 {
    match material {
        &Material::Light { color } => color,
        _ => Vec3::new(0.0, 0.0, 0.0),
    }
}
