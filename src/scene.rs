use std::collections::TreeMap;
use serialize::json::ToJson;
use std::str;
use serialize::json::Json;
use serialize::json::{Object, List};
//use serialize::{json, Encodable, Decodable};
use serialize::json;
use std::io::File;
use std::sync::Arc;
use image_types::Color;
use cgmath::{EuclideanVector, Point, Vector};
use cgmath::{Vector3, Point3, Ray3, Ray};
use cgmath::{dot};

struct Sphere {
    pos: Point3<f32>,
    radius: f32
}

pub struct Scene {
    objects: Vec<SceneObject>,
    lights: Vec<SceneLight>
}

struct Material {
    color: Color
}

struct SceneObject {
    material: Arc<Material>,
    geometry: Box<Intersectable+Send+Sync+'static>
}

struct SceneLight {
    pos: Point3<f32>,
    color: Color,
    intensity: f32
}
pub fn build_scene(filename: &str) -> Scene {
    let path = Path::new(filename);
    let contents = File::open(&path).read_to_end();
    let content = 
        match contents {
            Err(err) => fail!("Error reading {}: {}", path.display(), err),
            Ok(text) => text
        };
    let content_string = str::from_utf8(content.as_slice())
                         .expect("Couldn't unwrap string as UTF-8");
    let json_object = json::from_str(content_string).unwrap();

    let mut materials;
    let mut objects;
    match json_object {
        Object(contents) => {
            let material_json = contents.find(&"materials".to_string())
                                        .expect("JSON missing materials section.");
            materials = parse_materials(material_json);

            let objects_json = contents.find(&"objects".to_string())
                                       .expect("JSON missing objects section.");
            objects = parse_objects(objects_json);
        }
        _ => fail!("Error, top level of scene file isn't an object.")
    }
    
    let mat1 = Arc::new(Material { color: Color { r: 0.9, g: 0.9, b: 0.9 } });
    let mut objs = vec![ 
        SceneObject { geometry: box Sphere::new((0.0, 0.0, -101.0), 100.0),
                      material: mat1.clone() }];
    for i in range(0u, 10) {
        let angle = i as f32 / 5.0 * 3.141592654;
        objs.push( SceneObject { geometry: box Sphere::new((angle.sin(), angle.cos(), -1.0), 0.3),
                                 material: mat1.clone() });
    }
    let lights = vec![
        SceneLight { pos: Point3::new(2.0, -1.0, 2.0),
                     color: Color { r: 1.0, g: 0.9, b: 0.6 },
                     intensity: 1.0 },
        SceneLight { pos: Point3::new(-2.0, -1.0, 3.0),
                     color: Color { r: 0.8, g: 0.9, b: 1.0 },
                     intensity: 1.0 }
        ];
                  
    Scene { objects: objects,
            lights: lights}
}

fn parse_materials(materials_json: &Json) -> TreeMap<String, String> {
    let mut material_map = TreeMap::new();
    let materials = materials_json.as_list()
                                  .expect("Materials isn't a list");
        
    for material in materials.iter() {
        let (name, mat) = parse_mat(material);
        material_map.insert(name, mat);
    }
        
    material_map
}

fn parse_mat(material_json: &Json) -> (String, String) {
    let name = material_json.find(&"name".to_string())
                            .expect("Material missing name")
                            .as_string()
                            .expect("Name is not a string");
    
    let color = material_json.find(&"color".to_string())
                             .expect("Material missing color")
                             .as_list()
                             .expect("Color not of format [r, g, b]");

    (name.to_string(), format!("{}", color))
}

fn parse_objects(objects_json: &Json) -> Vec<SceneObject> {
    let objects = objects_json.as_list()
                              .expect("Objects isn't a list");
    let mut scene_objects = Vec::with_capacity(objects.len());
    for object in objects.iter() {
        let obj = parse_obj(object);
        scene_objects.push(obj);
    }
    scene_objects
}

fn parse_obj(object_json: &Json) -> SceneObject {
    let object = object_json.as_object()
                            .expect("Object isn't json");
    let object_type = object.find(&"type".to_string())
                            .expect("Object doesn't have a type")
                            .as_string()
                            .expect("Object type isn't a string");
    if object_type.as_slice() != "sphere" {
        fail!("Only spheres are currently supported!");
    }
    let material = object.find(&"material".to_string())
                         .expect("Object doesn't have a material")
                         .as_string()
                         .expect("Object material isn't a string");
    let pos = object.find(&"position".to_string())
                    .expect("Object doesn't have a position")
                    .as_list()
                    .expect("Object position isn't of form [x, y, z]");
    let radius = object.find(&"radius".to_string())
                       .expect("Object doesn't have a radius")
                       .as_f64()
                       .expect("Object radius isn't a number") as f32;
    let x = pos[0].as_f64().expect("Position should only contain numbers") as f32;
    let y = pos[1].as_f64().expect("Position should only contain numbers") as f32;
    let z = pos[2].as_f64().expect("Position should only contain numbers") as f32;
    SceneObject { geometry: box Sphere::new((x, y, z), radius),
                  material: Arc::new(Material{ color: Color { r: 1.0, g: 1.0, b: 1.0} }) }
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
