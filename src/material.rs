use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::prelude::*;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f32 },
    //Dielectric {},
}

impl Default for Material {
    fn default() -> Self {
        Material::Lambertian {
            albedo: Vec3::default(),
        }
    }
}

pub fn scatter(
    material: &Material,
    ray_in: &Ray,
    hit_record: &HitRecord,
    attenuation: &mut Vec3,
    scattered: &mut Ray,
) -> bool {
    match material {
        &Material::Lambertian { albedo } => {
            let target = hit_record.p() + hit_record.normal() + random_in_unit_sphere();
            *scattered = Ray::new(hit_record.p(), target - hit_record.p());
            *attenuation = albedo;
            return true;
        }
        &Material::Metal { albedo, fuzz } => {
            let reflected = reflect(
                &Vec3::unit_vector(&ray_in.direction()),
                &hit_record.normal(),
            );
            *scattered = Ray::new(hit_record.p(), reflected + fuzz * random_in_unit_sphere());
            *attenuation = albedo;
            return Vec3::dot(&scattered.direction(), &hit_record.normal()) > 0.0;
        }
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * Vec3::dot(v, n) * *n
}

fn random_in_unit_sphere() -> Vec3 {
    let mut p = Vec3::default();
    let mut rng = rand::thread_rng();

    loop {
        p = 2.0 * Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
            - Vec3::new(1.0, 1.0, 1.0);

        if p.squared_length() < 1.0 {
            return p;
        }
    }
}
