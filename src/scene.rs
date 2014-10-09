extern crate cgmath;

use std::rc::Rc;
use image_types::Color;
use cgmath::{EuclideanVector, Point};
use cgmath::{Vector3, Point3, Ray3};
use cgmath::{dot};

#[deriving(Show, Clone)]
struct Sphere {
    pos: Point3<f32>,
    radius: f32
}

#[deriving(Clone)]
pub struct Scene {
    objects: Vec<SceneObject>
}

#[deriving(Clone)]
struct Material {
    color: Color
}

#[deriving(Clone)]
struct SceneObject {
    material: Material,
    geometry: Sphere
}

enum Geometry {
    SphereGeometry(Sphere)
}

struct SceneLight {
    pos: Point3<f32>,
    color: Color,
    intensity: f32
}

pub fn build_scene(filename: &str) -> Scene {
    let mat1 = Material { color: Color { r: 0.9, g: 0.9, b: 0.9 } };
    let mat2 = Material { color: Color { r: 1.0, g: 0.0, b: 0.4 } };
    let objs = vec![SceneObject { geometry: Sphere::new((0.0, 0.0, 2.0), 1.0),
                                  material: mat1 },
                    SceneObject { geometry: Sphere::new((0.0, 2.0, 4.0), 3.0),
                                  material: mat2 }];
                  
    Scene { objects: objs }
}

impl Scene {
    pub fn trace_ray(&self, ray: &Ray3<f32>) -> Color {
        let mut closest = None;
        let mut closest_distance = 99999999999.0;
        for object in self.objects.iter() {
            let result = object.geometry.intersection(ray);
            match result {
                Some(distance) => {
                    if distance < closest_distance {
                        closest = Some(object);
                        closest_distance = distance;
                    }
                },
                None => ()
            }
        }
        match closest {
            Some(object) => object.material.color,
            None         => sky_color(&ray.direction)
        }
        
    }
}

fn sky_color(direction: &Vector3<f32>) -> Color {
    Color { r: 0.0, g: 0.0, b: 0.0 }
}

impl Sphere {
    fn new(origin: (f32, f32, f32), radius: f32) -> Sphere {
        let (x, y, z) = origin;
        Sphere { pos: Point3 {x: x, y: y, z: z},
                 radius: radius }
    }
}

impl Intersectable for Sphere {
    fn intersection(&self, ray: &Ray3<f32>) -> Option<f32> {
        /* See http://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection */
        let delta = self.pos.sub_p(&ray.origin);
        let a = ray.direction.length2(); // Square magnitude
        let b = 2.0 * dot(ray.direction, delta);
        let c = delta.length2() - self.radius*self.radius;
        let discriminant = b*b - 4.0*a*c;
        if discriminant >= 0.0 {
            let distance = (-b - discriminant.sqrt()) / (2.0 * a);
            Some(distance)
        } else {
            None
        }
    }
}

struct Intersection {
    point: Point3<f32>,
    normal: Vector3<f32>
}

trait Intersectable {
    fn intersection(&self, ray: &Ray3<f32>) -> Option<f32>;
}


