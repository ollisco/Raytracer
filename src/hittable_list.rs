use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::sphere::Sphere;

pub struct HittableList {
    pub list: Vec<Sphere>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut closest_yet = t_max;

        for object in &self.list {
            if let Some(hit_rec) = object.hit(ray, t_min, closest_yet) {
                closest_yet = hit_rec.t;
                hit_record = Some(hit_rec);
            }
        }
        hit_record
    }
}
