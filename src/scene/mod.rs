use parse_scene::parse_scene;
use std::sync::Arc;
use image_types::Color;
use cgmath::{Point, Vector};
use cgmath::{Vector3, Point3, Ray3, Ray};
use cgmath::dot;
use self::util::random_cos_around;
pub use self::illuminator::Illuminator;
pub use self::intersectable::Intersectable;
pub use self::scene_objects::{SceneObject, Sphere};
pub use self::scene_lights::{SceneLight, PointLight, DirectionalLight};

mod util;
mod illuminator;
mod intersectable;
mod scene_objects;
mod scene_lights;

pub struct Scene {
    pub objects: Vec<SceneObject>,
    pub lights: Vec<SceneLight>,
    pub num_gi_samples: u32,
    pub num_shadow_samples: u32,
    pub bounces: u32
}

pub struct Material {
    pub color: Color
}

pub fn build_scene(filename: &str) -> Scene {
    let scene = parse_scene(filename);
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
        let mut total_light = Color { r: 0.0, g: 0.0, b: 0.0 };
        let reduced_samples = self.num_gi_samples >> (depth * 2) as uint;
        if reduced_samples == 0 { return Color { r: 0.0, g: 0.0, b: 0.0 }; };
        for _ in range(0, reduced_samples) {
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
        total_light.mul_s(1.0/reduced_samples as f32)
    }
}

fn sky_color(direction: &Vector3<f32>) -> Color {
    let fac = (dot(*direction, Vector3::unit_z()) + 1.0) * 0.5;
    Color { r: 0.0, g: 0.0, b: fac }
}

struct Intersection {
    point: Point3<f32>,
    normal: Vector3<f32>,
    material: Arc<Material>
}
