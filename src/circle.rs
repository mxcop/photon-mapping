use glam::Vec2;

use crate::ray::Ray;

#[derive(Clone, Copy)]
pub struct Circle {
    pub o: Vec2,
    pub r: f32
}

impl Circle {
    pub fn new(origin: Vec2, radius: f32) -> Self {
        Self {
            o: origin,
            r: radius
        }
    }

    pub fn intersect_ray(&self, ray: Ray) -> Option<f32> {
        let oc = self.o - ray.o;
        let a = ray.d.dot(ray.d);
        let b = 2.0 * ray.d.dot(oc);
        let c = oc.dot(oc) - self.r * self.r;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant >= 0.0 {
            let sqrt_discriminant = discriminant.sqrt();
            let t = (-b - sqrt_discriminant) / (2.0 * a);

            if t > 0.0 {
                return Some(t);
            }
        }

        None
    }

    pub fn intersect_point(&self, p: Vec2) -> bool {
        self.o.distance(p) < self.r
    }
}
