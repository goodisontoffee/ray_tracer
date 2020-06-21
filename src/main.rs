mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hittable::Hittable;
use hittable_list::HittableList;
use rand::prelude::*;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

fn color(r: &Ray, world: &HittableList) -> Vec3 {
    match world.hit(&r, 0.0, std::f32::MAX) {
        Some(hit_record) => {
            0.5 * Vec3::new(
                hit_record.normal().x() + 1.0,
                hit_record.normal().y() + 1.0,
                hit_record.normal().z() + 1.0,
            )
        }
        None => {
            let unit_direction = Vec3::unit_vector(&r.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        }
    }
}

fn main() {
    let width = 200;
    let height = 100;
    let number_of_samples = 100;
    let max_value = 255;

    let mut list: Vec<Box<dyn Hittable>> = Vec::new();
    list.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    list.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));
    let world = HittableList::new(list);

    let cam = Camera::new();
    let mut rng = rand::thread_rng();

    println!("P3\n{} {}\n{}", width, height, max_value);

    for j in (0..height).rev() {
        for i in 0..width {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..number_of_samples {
                let u = (i as f32 + rng.gen::<f32>()) / width as f32;
                let v = (j as f32 + rng.gen::<f32>()) / height as f32;

                let r = &cam.get_ray(u, v);
                col = col + color(&r, &world);
            }

            let col = col / number_of_samples as f32;

            let ir = (255.99 * col.r()) as i32;
            let ig = (255.99 * col.g()) as i32;
            let ib = (255.99 * col.b()) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
