use parse_scene::parse_scene;
use std::sync::Arc;
use image_types::Color;
use cgmath::{EuclideanVector, Point, Vector};
use cgmath::{Vector3, Point3, Ray3, Ray};
use cgmath::{dot};

pub struct Sphere {
    pub pos: Point3<f32>,
    pub radius: f32
}

pub struct Scene {
    objects: Vec<SceneObject>,
    lights: Vec<SceneLight>
}

pub struct Material {
    pub color: Color
}

pub struct SceneObject {
    pub material: Arc<Material>,
    pub geometry: Box<Intersectable+Send+Sync+'static>
}

pub struct SceneLight {
    pub pos: Point3<f32>,
    pub color: Color,
    pub intensity: f32
}

pub fn build_scene(filename: &str) -> Scene {
    let (objects, lights) = parse_scene(filename);
    Scene { objects: objects,
            lights: lights}
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
                let diff = self.light_diffuse(&intersection.point,
                                              &intersection.normal);
                
                diff.mul_c(&object.material.color)
            }
            None         => sky_color(&ray.direction)
        }
    }

    pub fn check_ray(&self, ray: &Ray3<f32>) -> bool {
        for object in self.objects.iter() {
            match object.geometry.intersection(ray) {
                Some(_) => return true,
                None    => ()
            }
        }
        false
    }

    pub fn check_ray_distance(&self, ray: &Ray3<f32>, distance: f32) -> bool {
        for object in self.objects.iter() {
            match object.geometry.intersection(ray) {
                Some(d) if d >= distance => return true,
                _                        => ()
            }
        }
        false
    }

    pub fn light_diffuse(&self, point: &Point3<f32>, normal: &Vector3<f32>) -> Color {
        let mut total_light = Color { r: 0.0, g: 0.0, b: 0.0 };
        for light in self.lights.iter() {
            let light_direction = light.pos.sub_p(&Point::origin()).normalize();
        
            if !self.check_ray(&Ray::new(*point, light_direction)) {
                let diff = saturate(dot(*normal, light_direction));
                total_light = total_light.add_c(&light.color.mul_s(light.intensity * diff));
                    
            }
        }
        total_light
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

    fn intersection_info(&self, point: &Point3<f32>) -> Intersection {
        let normal = point.sub_p(&self.pos).normalize();
        
        Intersection { point: point.add_v(&normal.mul_s(0.000001)),
                       normal: normal }
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
