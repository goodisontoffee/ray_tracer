use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = Vec3::dot(&r.direction(), &r.direction());
        let b = 2.0 * Vec3::dot(&oc, &r.direction());
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let hit_record = HitRecord::new(
                    temp,
                    r.point_at_parameter(temp),
                    (r.point_at_parameter(temp) - self.center) / self.radius,
                );
                return Some(hit_record);
            }
            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let hit_record = HitRecord::new(
                    temp,
                    r.point_at_parameter(temp),
                    (r.point_at_parameter(temp) - self.center) / self.radius,
                );
                return Some(hit_record);
            }
        }

        None
    }
}
