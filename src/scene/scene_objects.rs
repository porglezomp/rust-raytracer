use std::sync::Arc;
use cgmath::{EuclideanVector, Point, Vector};
use cgmath::{Point3, Ray3};
use cgmath::dot;
use scene::{Intersectable, Material, Intersection};

pub struct Sphere {
    pos: Point3<f32>,
    radius: f32,
}

pub struct SceneObject {
    pub material: Arc<Material>,
    pub geometry: Box<Intersectable+Send+Sync+'static>
}

impl SceneObject {
    pub fn intersection(&self, ray: &Ray3<f32>) -> Option<f32> {
        self.geometry.intersection(ray)
    }

    pub fn intersection_info(&self, point: &Point3<f32>) -> Intersection {
        self.geometry.intersection_info(point, self)
    }
}

impl Sphere {
    pub fn new(origin: (f32, f32, f32), radius: f32) -> Sphere {
        let (x, y, z) = origin;
        Sphere { pos: Point3 {x: x, y: y, z: z},
                 radius: radius }
    }
}

impl Intersectable for Sphere {
    fn intersection(&self, ray: &Ray3<f32>) -> Option<f32> {
        // Optimized ray-sphere intersection
        // See http://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection
        let delta = ray.origin.sub_p(&self.pos);
        // Skip multiplying by two
        let b = dot(ray.direction, delta);
        let c = delta.length2() - self.radius*self.radius;
        // Optimized discriminant, our b in normal b/2, which means that
        // we have normal (b^2)/4 as our b^2, so we can not multiply c by 4
        let discriminant = b*b - c;
        if discriminant >= 0.0 {
            // Our b is half the normal b, so we don't have to divide by 2
            let distance = -b - discriminant.sqrt();
            if distance > 0.0 {
                return Some(distance);
            }
        }
        None
    }

    fn intersection_info(&self, point: &Point3<f32>, object: &SceneObject) -> Intersection {
        let normal = point.sub_p(&self.pos).normalize();
        
        Intersection { point: point.add_v(&normal.mul_s(0.000001)),
                       normal: normal,
                       material: object.material.clone() }
    }
}
