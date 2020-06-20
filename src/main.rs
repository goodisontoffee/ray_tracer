mod ray;
mod vec3;

use ray::Ray;
use vec3::Vec3;

fn color(r: &Ray) -> Vec3 {
    let unit_direction = Vec3::unit_vector(&r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    let nx = 200;
    let ny = 100;
    let max_value = 255;

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    println!("P3\n{} {}\n{}", nx, ny, max_value);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;

            let r = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            let col = color(&r);

            let b: f32 = 0.2;

            let ir = (255.99 * col.x()) as i32;
            let ig = (255.99 * col.y()) as i32;
            let ib = (255.99 * col.z()) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
