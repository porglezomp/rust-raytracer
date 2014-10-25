use scene::{Intersection, SceneObject};
use cgmath::{Ray3, Point3};

pub trait Intersectable {
    fn intersection(&self, ray: &Ray3<f32>) -> Option<f32>;
    fn intersection_info(&self, point: &Point3<f32>, object: &SceneObject) -> Intersection;
}
