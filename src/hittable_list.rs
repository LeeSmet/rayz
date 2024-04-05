#![allow(dead_code)]

use std::rc::Rc;

use crate::{hittable::Hittable, interval::Interval};

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object)
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &crate::ray::Ray, ray_t: Interval) -> Option<crate::hittable::HitRecord> {
        let mut hit_record = None;
        // Keep track of the closest hit so far, which is returned as part of a possible HitRecord.
        let mut closest_so_far = ray_t.max();

        for obj in &self.objects {
            // Pas the current closest hit distance as upper bound on where we want to find hits,
            // so _IF_ we get a hit, it _MUST_ be closer than the previous closest hit.
            if let Some(hr) = obj.hit(ray, Interval::new(ray_t.min(), closest_so_far)) {
                closest_so_far = hr.t;
                hit_record = Some(hr);
            }
        }

        hit_record
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}
