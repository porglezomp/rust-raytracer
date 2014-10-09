extern crate cgmath;

use image_types::Color;
use cgmath::{EuclideanVector};
use cgmath::{Point3, Ray3, Vector3};

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
    Scene { objects: vec![] }
}

impl Scene {
    pub fn trace_ray(&self, ray: &Ray3<f32>) -> Color {
        Color { r: 1.0, g: 1.0, b: 1.0 }
    }
}
