extern crate cgmath;

use image_types::Color;
use cgmath::{EuclideanVector, Point, Vector, BaseFloat};
use cgmath::{Vector3, Point3, Ray3};
use cgmath::{dot};

#[deriving(Show)]
struct Sphere {
    pos: Point3<f32>,
    radius: f32
}

pub struct Scene {
    objects: Vec<SceneObject>
}

struct Material {
    color: Color
}

struct SceneObject {
    material: Material,
    geometry: Box<Intersectable+Send+Sync+'static>
}

struct SceneLight {
    pos: Point3<f32>,
    color: Color,
    intensity: f32
}

pub fn build_scene(filename: &str) -> Scene {
    let mat1 = Material { color: Color { r: 0.9, g: 0.9, b: 0.9 } };
    let mat2 = Material { color: Color { r: 1.0, g: 0.0, b: 0.4 } };
    let objs = vec![SceneObject { geometry: box Sphere::new((0.0, 0.0, 0.0), 1.0),
                                  material: mat1 }];
 //                   SceneObject { geometry: box Sphere::new((0.0, 2.0, 4.0), 3.0),
   //                               material: mat2 }];
                  
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
            Some(object) => {
                let point = ray.origin.add_v(&ray.direction.mul_s(closest_distance));
                let intersection = object.geometry.intersection_info(&point);
                let light_vector = Vector3::new(-1.0, -1.0, 1.0).normalize();
                let l = saturate(dot(intersection.normal, light_vector));
                let Color { r, g, b } = object.material.color;
                Color { r: r*l, g: g*l, b: b*l }
            }
            None         => sky_color(&ray.direction)
        }
        
    }
}

fn saturate(x: f32) -> f32 {
    match x {
        _ if x < 0.0 => 0.0,
        _ if x > 1.0 => 1.0,
        x            => x
    }
}

fn sky_color(direction: &Vector3<f32>) -> Color {
    let fac = (dot(*direction, Vector3::unit_z()) + 1.0) * 0.5;
    Color { r: 0.0, g: 0.0, b: fac }
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

    fn intersection_info(&self, point: &Point3<f32>) -> Intersection {
        let normal = point.sub_p(&self.pos).normalize();
        Intersection { point: point.clone(), normal: normal }
    }
}

struct Intersection {
    point: Point3<f32>,
    normal: Vector3<f32>
}

trait Intersectable {
    fn intersection(&self, ray: &Ray3<f32>) -> Option<f32>;
    fn intersection_info(&self, point: &Point3<f32>) -> Intersection;
}

//trait Light {}
