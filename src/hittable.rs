use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    t: f32,
    p: Vec3,
    normal: Vec3,
}

impl HitRecord {
    pub fn new(t: f32, p: Vec3, normal: Vec3) -> Self {
        HitRecord { t, p, normal }
    }

    pub fn t(&self) -> f32 {
        self.t
    }

    pub fn p(&self) -> Vec3 {
        self.p
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        None
    }
}
