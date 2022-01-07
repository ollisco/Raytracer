use crate::hittable::{HitRecord, Hittable};
use crate::material::{self, Material};
use crate::ray::Ray;
use macaw::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // Using formula for sphere intersection
        // (P−C)⋅(P−C)=r2, P(t) = A+tb
        // (P(t)−C)⋅(P(t)−C)=r2
        // (A+tb−C)⋅(A+tb−C)=r2
        // t2b⋅b+2tb⋅(A−C)+(A−C)⋅(A−C)−r2=0

        let origin_center_dist = ray.origin() - self.center;
        let ray_dir = ray.direction();

        let a = ray_dir.length_squared();
        let half_b = origin_center_dist.dot(ray_dir);
        let c = origin_center_dist.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        };

        let sqrt_discriminant = discriminant.sqrt();
        let root = (-half_b - sqrt_discriminant) / a;

        if root < t_min || root > t_max {
            let root = (-half_b + sqrt_discriminant) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        // Hidden behavior, needs cleanup
        /*
        hit_rec.t = root;
        hit_rec.point = ray.at(hit_rec.t);
        let outward_normal = (hit_rec.point - self.center) / self.radius;
        hit_rec.set_face_normal(ray, outward_normal);
        hit_rec.material = self.material.clone();
        */
        let outward_normal = (ray.at(root) - self.center) / self.radius;
        let mut new_hit_record = HitRecord {
            point: ray.at(root),
            t: root,
            u: 0.0,
            v: 0.0,
            normal: outward_normal,
            front_face: true,
            material: self.material.clone(),
        };

        new_hit_record.set_face_normal(ray, outward_normal);

        Some(new_hit_record)
    }
}
