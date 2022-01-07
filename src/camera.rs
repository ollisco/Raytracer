use crate::ray::Ray;
use crate::util::*;
use macaw::Vec3;

pub struct Camera {
    //Todo: Cleanup public
    pub aspect_ratio: f32,
    pub viewport_height: f32,
    pub viewport_width: f32,
    pub focal_length: f32,

    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    bottom_left_corner: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov_degrees: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_distance: f32,
    ) -> Self {
        let angle = degrees_to_radians(vfov_degrees);
        let z_hight = (angle / 2.0).tan();
        let viewport_height = 2.0 * z_hight;
        let viewport_width = viewport_height * aspect_ratio;

        let w = (look_from - look_at).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let focal_length = 1.0_f32;

        let origin = look_from;
        let horizontal = viewport_width * u * focus_distance;
        let vertical = viewport_height * v * focus_distance;
        let bottom_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_distance;

        let lens_radius = aperture / 2.0;
        Self {
            aspect_ratio,
            viewport_height,
            viewport_width,
            focal_length,

            origin,
            horizontal,
            vertical,
            bottom_left_corner,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = Vec3::new(u * rd.x, v * rd.y, rd.z);
        let direction =
            self.bottom_left_corner + u * self.horizontal + v * self.vertical - self.origin;
        Ray::new(self.origin + offset, direction - offset)
    }
}
