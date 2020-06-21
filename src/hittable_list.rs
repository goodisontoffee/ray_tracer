use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct HittableList {
    list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(list: Vec<Box<dyn Hittable>>) -> Self {
        HittableList { list }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut result = None;
        let mut closest_so_far = t_max;

        for object in &self.list {
            if let Some(hit_record) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = hit_record.t();
                result = Some(hit_record);
            }
        }

        result
    }
}
