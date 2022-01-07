use macaw::Vec3;
use rand::Rng;
use std::f32::consts;

macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = min!($($z),*);
        if $x < y {
            $x
        } else {
            y
        }
    }}
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    // clamps x to range [min, max]

    if x < min {
        return min;
    };
    if x > max {
        return max;
    };
    x
}

pub fn write_pixel_color(pixel_color: Vec3, samples_per_pixel: u32) {
    let scale = 1.0 / (samples_per_pixel as f32);
    let (r, g, b) = (
        (pixel_color.x * scale).sqrt(),
        (pixel_color.y * scale).sqrt(),
        (pixel_color.z * scale).sqrt(),
    );

    println!(
        "{} {} {}",
        ((256_f32 * clamp(r, 0.0, 0.999)) as u16),
        ((256_f32 * clamp(g, 0.0, 0.999)) as u16),
        ((256_f32 * clamp(b, 0.0, 0.999)) as u16),
    );
}

pub fn random_f32() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen::<f32>()
}

pub fn random_range(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

pub fn random_vec() -> Vec3 {
    Vec3::new(random_f32(), random_f32(), random_f32())
}

pub fn random_vec_range(min: f32, max: f32) -> Vec3 {
    Vec3::new(
        random_range(min, max),
        random_range(min, max),
        random_range(min, max),
    )
}

pub fn random_vec_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = 2.0 * Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>());
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    random_vec_in_unit_sphere().normalize()
}

pub fn random_vec_in_hemisphere(normal: &Vec3) -> Vec3 {
    let vec_in_unitsphere = random_vec_in_unit_sphere();
    if vec_in_unitsphere.dot(*normal) > 0.0 {
        return vec_in_unitsphere;
    } else {
        return -vec_in_unitsphere;
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3::new(random_range(-1.0, 1.0), random_range(-1.0, 1.0), 0.0);
        if p.length_squared() > 1.0 {
            continue;
        }
        return p;
    }
}

pub fn vec_near_zero(vec: Vec3) -> bool {
    let margin = 1e-8;
    vec.x < margin && vec.y < margin && vec.z < margin
}

pub fn vec_reflect(incoming_vec: &Vec3, normal_unitvec: &Vec3) -> Vec3 {
    *incoming_vec - 2.0 * incoming_vec.dot(*normal_unitvec) * *normal_unitvec
}

pub fn vec_refract(uv: &Vec3, normal: &Vec3, refractive_index_quotion: f32) -> Vec3 {
    // both uv and normal are unit vectors
    // refractive_index_quotion = incoming index / outgoing index
    let uv_dot_normal = normal.dot(-(*uv));
    let angle_between;
    if uv_dot_normal < 1.0 {
        angle_between = uv_dot_normal;
    } else {
        angle_between = 1.0;
    }
    let out_perpendicular = refractive_index_quotion * ((*uv) + (angle_between * (*normal)));
    let out_parallel = (*normal) * -(((1.0 - out_perpendicular.length_squared()).abs()).sqrt());
    out_perpendicular + out_parallel
}

pub fn vec_refract2(uv: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = min!(n.dot(-*uv), 1.0);
    let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * *n;
    r_out_perp + r_out_parallel
}

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * (consts::PI / 180.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_macro() {
        assert!(min!(1.0, 2.0, 1.6, 1.4) < 1.5);
    }

    #[test]
    fn test_dot() {
        let x = Vec3::new(1.0, 2.0, 3.0);
        let t = Vec3::new(0.5, 1.5, 2.5);
        assert_eq!(x.dot(t), t.dot(x));
    }

    #[test]
    fn test_refract() {
        let uv = Vec3::new(0.0, 0.0, -1.0);
        let normal = Vec3::new(0.0, 0.0, 1.0);
        let q = 1.0_f32;
        println!("{:?}", vec_refract(&uv, &normal, q));
    }
}
