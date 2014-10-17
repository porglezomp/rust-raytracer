use parse_scene::parse_scene;
use std::sync::{Arc, Weak};
use image_types::Color;
use cgmath::{EuclideanVector, Point, Vector};
use cgmath::{Vector3, Point3, Ray3, Ray};
use cgmath::{dot};
use std::rand;
use std::rand::distributions::{Normal, IndependentSample};

pub struct Sphere {
    pos: Point3<f32>,
    radius: f32,
}

pub struct Scene {
    pub objects: Vec<SceneObject>,
    pub lights: Vec<SceneLight>,
    pub num_samples: u32,
    pub bounces: u32
}

pub struct Material {
    pub color: Color
}

pub struct SceneObject {
    pub material: Arc<Material>,
    pub geometry: Box<Intersectable+Send+Sync+'static>
}

pub struct PointLight {
    pub position: Point3<f32>,
    pub color: Color,
    pub intensity: f32,
    pub radius: f32
}

pub struct DirectionalLight {
    pub direction: Vector3<f32>,
    pub color: Color,
    pub intensity: f32,
    pub angle: f32
}

pub struct SceneLight {
    pub illuminator: Box<Illuminator+Send+Sync+'static>
}

pub fn build_scene(filename: &str) -> Scene {
    let mut scene = parse_scene(filename);
    scene
}

impl Scene {
    pub fn trace_ray(&self, ray: &Ray3<f32>, depth: u32) -> Color {
        let intersect = self.find_intersection(ray);
        match intersect {
            Some(intersection) => {
                let diff = self.light_diffuse(&intersection.point,
                                              &intersection.normal,
                                              depth);
                
                diff.mul_c(&intersection.material.color)
            }
            None         => sky_color(&ray.direction)
        }
    }

    fn find_intersection(&self, ray: &Ray3<f32>) -> Option<Intersection> {
        let mut closest = None;
        let mut closest_distance = 99999999999.0;
        for object in self.objects.iter() {
            let result = object.intersection(ray);
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
                let intersection = object.intersection_info(&point);
                Some(intersection)
            },
            None => None
        }
    }

    pub fn check_ray(&self, ray: &Ray3<f32>) -> bool {
        for object in self.objects.iter() {
            match object.intersection(ray) {
                Some(_) => return true,
                None    => ()
            }
        }
        false
    }

    pub fn check_ray_distance(&self, ray: &Ray3<f32>, distance: f32) -> bool {
        for object in self.objects.iter() {
            match object.intersection(ray) {
                Some(d) if d <= distance => return true,
                _ => ()
            }
        }
        false
    }

    pub fn light_diffuse(&self, point: &Point3<f32>, normal: &Vector3<f32>, depth: u32) -> Color {
        let mut total_light = Color { r: 0.0, g: 0.0, b: 0.0 };
        for light in self.lights.iter() {
            total_light = total_light.add_c(&light.illuminate(self, point, normal));
        }
        if depth < self.bounces {
            total_light = total_light.add_c(&self.environment_light(point, normal, depth + 1));
        }
        total_light
    }

    fn environment_light(&self, point: &Point3<f32>, normal: &Vector3<f32>, depth: u32) -> Color {
        if self.num_samples == 0 { return Color { r: 0.0, g: 0.0, b: 0.0 }; };
        let mut total_light = Color { r: 0.0, g: 0.0, b: 0.0 };
        for _ in range(0, self.num_samples) {
            let vector = random_cos_around(normal);
            match self.find_intersection(&Ray::new(*point, vector)) {
                Some(intersection) => {
                    let incoming = self.light_diffuse(&intersection.point,
                                                      &intersection.normal,
                                                      depth);
                    total_light = total_light.add_c(&incoming);
                },
                None => total_light = total_light.add_c(&sky_color(&vector))
            }
        }
        total_light.mul_s(1.0/self.num_samples as f32)
    }
}

fn random_unit_vector() -> Vector3<f32> {
    let normal = Normal::new(0.0, 1.0);
    let x = normal.ind_sample(&mut rand::task_rng()) as f32;
    let y = normal.ind_sample(&mut rand::task_rng()) as f32;
    let z = normal.ind_sample(&mut rand::task_rng()) as f32;
    Vector3::new(x, y, z).normalize()
}

fn random_cos_around(vector: &Vector3<f32>) -> Vector3<f32> {
    let second = random_unit_vector();
    vector.add_v(&second).normalize()
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

impl SceneObject {
    fn intersection(&self, ray: &Ray3<f32>) -> Option<f32> {
        self.geometry.intersection(ray)
    }

    fn intersection_info(&self, point: &Point3<f32>) -> Intersection {
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

impl Illuminator for SceneLight {
    fn illuminate(&self, scene: &Scene, point: &Point3<f32>, normal: &Vector3<f32>) -> Color {
        let off_surface_point = &point.add_v(&normal.mul_s(0.0001));
        self.illuminator.illuminate(scene, off_surface_point, normal)
    }
}

impl Illuminator for DirectionalLight {
    fn illuminate(&self, scene: &Scene, point: &Point3<f32>, normal: &Vector3<f32>) -> Color {
        if !scene.check_ray(&Ray::new(*point, self.direction)) {
            let diff = saturate(dot(*normal, self.direction));
            let color = self.color.mul_s(self.intensity * diff);
            color
        } else {
            Color { r: 0.0,
                    g: 0.0,
                    b: 0.0 }
        }
    }
}

impl Illuminator for PointLight {
    fn illuminate(&self, scene: &Scene, point: &Point3<f32>, normal: &Vector3<f32>) -> Color {
        let delta = self.position.sub_p(point);
        let distance = delta.length();
        let direction = delta.normalize();
        if !scene.check_ray_distance(&Ray::new(*point, direction), distance) {
            let brightness = self.intensity / (distance * distance);
            let diff = saturate(dot(*normal, direction));
            let color = self.color.mul_s(brightness * diff);
            color
        } else {
            Color { r: 0.0,
                    g: 0.0,
                    b: 0.0 }
        }
    }
}

struct Intersection {
    point: Point3<f32>,
    normal: Vector3<f32>,
    material: Arc<Material>
}

pub trait Intersectable {
    fn intersection(&self, ray: &Ray3<f32>) -> Option<f32>;
    fn intersection_info(&self, point: &Point3<f32>, object: &SceneObject) -> Intersection;
}

pub trait Illuminator {
    fn illuminate(&self, scene: &Scene, point: &Point3<f32>, normal: &Vector3<f32>) -> Color;
}
