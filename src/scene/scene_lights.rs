use image_types::Color;
use cgmath::{EuclideanVector, Point, Vector, Rotation};
use cgmath::{Vector3, Point3, Ray, Basis3};
use cgmath::dot;
use scene::util::{random_unit_vector, random_in_cone, saturate};
use scene::{Illuminator, Scene};

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

impl Illuminator for SceneLight {
    fn illuminate(&self, scene: &Scene, point: &Point3<f32>, normal: &Vector3<f32>) -> Color {
        let off_surface_point = &point.add_v(&normal.mul_s(0.0001));
        self.illuminator.illuminate(scene, off_surface_point, normal)
    }
}

impl Illuminator for DirectionalLight {
    fn illuminate(&self, scene: &Scene, point: &Point3<f32>, normal: &Vector3<f32>) -> Color {
        let mut f = 0.0;
        let delta = 1.0 / scene.num_shadow_samples as f32;
        let rotation: Basis3<f32> = Rotation::between_vectors(&Vector3::unit_z(), &self.direction);
        for _ in range(0, scene.num_shadow_samples) {
            let vec = rotation.rotate_vector(&random_in_cone(self.angle));
            let ray = Ray::new(*point, vec);
            if !scene.check_ray(&ray) {
                f += delta;
            }
        }
        if f != 0.0 {
            let diff = saturate(dot(*normal, self.direction));
            let color = self.color.mul_s(self.intensity * diff * f);
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
        let mut flux = 0.0;
        for _ in range(0, scene.num_shadow_samples) {
            let delta = self.position.add_v(&random_unit_vector().mul_s(self.radius)).sub_p(point);
            let distance = delta.length();
            let direction = delta.normalize();
            if !scene.check_ray_distance(&Ray::new(*point, direction), distance) {
                flux += dot(*normal, direction) * self.intensity / (distance * distance);
            }
        }
        flux /= scene.num_shadow_samples as f32;
        let color = self.color.mul_s(flux);
        color
    }
}
