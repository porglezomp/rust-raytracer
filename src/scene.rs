extern crate cgmath;

use image_types::Color;
use cgmath::{EuclideanVector, Point};
use cgmath::{Vector3, Point3, Ray3};
use cgmath::{dot};

#[deriving(Show, Clone)]
struct Sphere {
    origin: Point3<f32>,
    radius: f32
}

#[deriving(Clone)]
pub struct Scene {
    objects: Vec<Sphere>
}

pub fn build_scene(filename: &str) -> Scene {
    let objs = vec![Sphere::new((0.0, 0.0, 2.0), 1.0),
                    Sphere::new((0.0, 2.0, 4.0), 3.0)];
                  
    Scene { objects: objs }
}

impl Scene {
    pub fn trace_ray(&self, ray: &Ray3<f32>) -> Color {
        for object in self.objects.iter() {
            if object.check_intersection(ray) {
                return Color { r: 1.0, g: 1.0, b: 1.0 }
            }
        }
        Color { r: 0.0, g: 0.0, b: 0.0 }
    }
}

impl Sphere {
    fn new(origin: (f32, f32, f32), radius: f32) -> Sphere {
        let (x, y, z) = origin;
        Sphere { origin: Point3 {x: x, y: y, z: z},
                 radius: radius }
    }
}

impl Intersectable for Sphere {
    fn check_intersection(&self, ray: &Ray3<f32>) -> bool {
        /* See http://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection */
        let delta = self.origin.sub_p(&ray.origin);
        let a = ray.direction.length2(); // Square magnitude
        let b = 2.0 * dot(ray.direction, delta);
        let c = delta.length2() - self.radius*self.radius;
        let discriminant = b*b - 4.0*a*c;
        if discriminant >= 0.0 {
            true
        } else {
            false
        }
    }

    fn intersection(&self, ray: &Ray3<f32>) -> Option<Intersection> {
        Some(Intersection { point: Point3 { x: 0.0, y: 0.0, z: 0.0 },
                            normal: Vector3 { x: 0.0, y: 0.0, z: 0.0 } })
    }
}

struct Intersection {
    point: Point3<f32>,
    normal: Vector3<f32>
}

trait Intersectable {
    fn check_intersection(&self, ray: &Ray3<f32>) -> bool;
    fn intersection(&self, ray: &Ray3<f32>) -> Option<Intersection>;
}
