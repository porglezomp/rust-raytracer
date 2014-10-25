use scene::{Sphere, SceneObject, Material};
use std::collections::TreeMap;
use serialize::json::Json;
use std::sync::Arc;

pub fn parse_objects(objects_json: &Json, materials: &TreeMap<String, Arc<Material>>) -> Vec<SceneObject> {
    let objects = objects_json.as_list()
                              .expect("Objects isn't a list");
    let mut scene_objects = Vec::with_capacity(objects.len());
    for object in objects.iter() {
        let obj = parse_obj(object, materials);
        scene_objects.push(obj);
    }
    scene_objects
}

fn parse_obj(object_json: &Json, materials: &TreeMap<String, Arc<Material>>) -> SceneObject {
    let object = object_json.as_object()
                            .expect("Object isn't a JSON object");
    let object_type = object.find(&"type".to_string())
                            .expect("Object doesn't have a type")
                            .as_string()
                            .expect("Object type isn't a string");
    if object_type.as_slice() != "sphere" {
        fail!("Only spheres are currently supported!");
    }
    let mat_name = object.find(&"material".to_string())
                         .expect("Object doesn't have a material")
                         .as_string()
                         .expect("Object material isn't a string");
    let material = materials.find(&mat_name.to_string())
                            .expect(format!("No material with name '{}'", mat_name).as_slice());
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
                  material: material.clone() }
}
