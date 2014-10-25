use scene::Scene;
use cgmath::{Point3, Vector3};
use image_types::Color;

pub trait Illuminator {
    fn illuminate(&self, scene: &Scene, point: &Point3<f32>, normal: &Vector3<f32>) -> Color;
}
