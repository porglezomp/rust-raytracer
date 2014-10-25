use cgmath::{EuclideanVector, Vector, Vector3};
use std::rand;
use std::rand::Rng;
use std::rand::distributions::{Normal, IndependentSample};

pub fn random_unit_vector() -> Vector3<f32> {
    let normal = Normal::new(0.0, 1.0);
    let x = normal.ind_sample(&mut rand::task_rng()) as f32;
    let y = normal.ind_sample(&mut rand::task_rng()) as f32;
    let z = normal.ind_sample(&mut rand::task_rng()) as f32;
    Vector3::new(x, y, z).normalize()
}

const PI : f32 = 3.141592653589793238;
pub fn random_in_cone(angle: f32) -> Vector3<f32> {
    // Generate a vector in the cone around <0, 0, 1>
    let max = 1.0;
    let min = (angle*PI/180.0).cos();
    let z = rand::task_rng().gen_range(min, max) as f32;
    let t = rand::task_rng().gen_range(0.0, 2.0*PI) as f32;
    let r = (1.0 - z*z).sqrt();
    let x = r * t.cos();
    let y = r * t.sin();
    let vec = Vector3::new(x, y, z);
    vec
}

pub fn random_cos_around(vector: &Vector3<f32>) -> Vector3<f32> {
    let second = random_unit_vector();
    vector.add_v(&second).normalize()
}

pub fn saturate(x: f32) -> f32 {
    match x {
        _ if x < 0.0 => 0.0,
        _ if x > 1.0 => 1.0,
        x            => x
    }
}
