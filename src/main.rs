mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod util;

use camera::Camera;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use macaw::Vec3;
use material::scatter;
use ray::Ray;
use rayon::prelude::*;
use sphere::Sphere;
use util::{random_f32, random_unit_vector, random_vec_in_hemisphere, write_pixel_color, *};

use crate::material::Material;

fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let ground_material = Material::Matte {
        albedo: Vec3::new(0.761, 0.698, 0.5),
    };
    world.list.push(Sphere::new(
        Vec3::new(0.0, -2000.0, 0.0),
        2000.0,
        ground_material,
    ));

    // let small_ball_radius = 0.2_f32;

    for a in -11..11 {
        for b in -11..11 {
            let small_ball_radius = random_range(0.1, 0.4);
            let choose_material = random_f32();
            let center = Vec3::new(
                (a as f32 + 0.9 * random_f32() + 0.1) * 2.0,
                random_range(small_ball_radius, 1.0), // sbm
                (b as f32 + 0.9 * random_f32() + 0.1) * 2.0,
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Material;

                if choose_material < 0.7 {
                    // 70 % chance of matte balls
                    let albedo = random_vec() * random_vec();
                    sphere_material = Material::Matte { albedo };
                    world
                        .list
                        .push(Sphere::new(center, small_ball_radius, sphere_material));
                } else if choose_material < 0.95 {
                    //15 % chance of metal balls
                    let albedo = random_vec_range(0.5, 1.0);
                    let fuzz = random_f32();
                    sphere_material = Material::Metal { albedo, fuzz };
                    world
                        .list
                        .push(Sphere::new(center, small_ball_radius, sphere_material));
                }
            }
        }
    }
    /*
    let big_material_a = Material::Metal {
        albedo: Vec3::new(0.5, 0.3, 0.5),
        fuzz: 0.0,
    };
    world
        .list
        .push(Sphere::new(Vec3::new(20.0, 3.5, 3.0), 4.0, big_material_a));
    */

    let big_material_b = Material::Metal {
        albedo: Vec3::new(0.831, 0.686, 0.215),
        fuzz: 0.3,
    };
    world
        .list
        .push(Sphere::new(Vec3::new(-2.0, 1.0, 0.5), 1.0, big_material_b));
    let big_material_c = Material::Metal {
        albedo: Vec3::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    };
    world
        .list
        .push(Sphere::new(Vec3::new(4.0, 1.5, 0.0), 1.5, big_material_c));

    world
}

fn ray_color(ray: &Ray, world: &HittableList, depth: u8) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if let Some(hit_rec) = world.hit(ray, 0.001, f32::MAX) {
        let mut scattered_ray = Ray::default();
        let mut attenuation = Vec3::default();
        if scatter(
            &hit_rec.material,
            ray,
            &hit_rec,
            &mut attenuation,
            &mut scattered_ray,
        ) {
            return attenuation * ray_color(&scattered_ray, world, depth - 1);
        }
        return Vec3::new(0.0, 0.0, 0.0);
    }
    if depth == 50 {
        let unit_direction = ray.direction().normalize();
        let transition_variable = 0.5 * (unit_direction.y + 1.0);
        let color1 = Vec3::new(1.0, 1.0, 1.0);
        let color2 = Vec3::new(0.294, 0.0, 0.039);

        // Using formula: Blended value = (1-t)*startvalue + t*endvalue
        return (1.0 - transition_variable) * color1 + transition_variable * color2;
    }

    return Vec3::new(1.0, 1.0, 1.0);
}

fn main() {
    // Image

    const ASPECT_RATIO: f32 = 3.0 / 2.0;
    const IMAGE_WIDTH: i32 = 1200;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
    const MAX_VALUE: u8 = 255;

    const SAMPLES_PER_PIXEL: u32 = 500;

    const MAX_DEPTH: u8 = 50; // Should be 50

    // World
    let world = random_scene();
    /* Previeous
    let material_ground = Material::Matte {
        albedo: Vec3::new(0.5, 0.5, 0.5),
    };
    let material_center = Material::Matte {
        albedo: Vec3::new(0.7, 0.3, 0.3),
    };
    let material_left = Material::Metal {
        albedo: Vec3::new(0.8, 0.8, 0.8),
        fuzz: 0.1,
    };
    let material_right = Material::Metal {
        albedo: Vec3::new(0.8, 0.6, 0.2),
        fuzz: 0.8,
    };

    let mut world = HittableList::new();
    world.list.push(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    // ground
    world.list.push(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));

    world.list.push(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.list.push(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));
    */

    // Camera
    let look_from = Vec3::new(0.0, 6.0, 30.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let focus_distance = 20.0;
    let aperture = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        focus_distance,
    );

    // Render

    println!("P3\n{} {}\n{}", IMAGE_WIDTH, IMAGE_HEIGHT, MAX_VALUE);

    for pixel_row in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Rows left: {}.", pixel_row);
        for pixel_column in 0..IMAGE_WIDTH {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..SAMPLES_PER_PIXEL {
                let u = (pixel_column as f32 + random_f32()) / (IMAGE_WIDTH) as f32;
                let v = (pixel_row as f32 + random_f32()) / (IMAGE_HEIGHT) as f32;
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, MAX_DEPTH);
            }
            write_pixel_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
}
