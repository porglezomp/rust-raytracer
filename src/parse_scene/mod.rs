use scene::{Scene, Sphere, SceneObject, Material};
use std::collections::TreeMap;
use std::str;
use serialize::json::{Json, Object};
use serialize::json;
use std::io::File;
use std::sync::Arc;
use image_types::Color;

mod lights;

pub fn parse_scene(filename: &str) -> Scene {
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
    let mut lights;
    match json_object {
        Object(contents) => {
            let material_json = contents.find(&"materials".to_string())
                                        .expect("JSON missing materials section.");
            materials = parse_materials(material_json);

            let objects_json = contents.find(&"objects".to_string())
                                       .expect("JSON missing objects section.");
            objects = parse_objects(objects_json, &materials);

            let lights_json = contents.find(&"lights".to_string())
                                      .expect("JSON missing lights section.");
            lights = lights::parse_lights(lights_json);
        }
        _ => fail!("Error, top level of scene file isn't an object.")
    }
        
    Scene { objects: objects,
            lights: lights,
            num_gi_samples: 16,
            num_shadow_samples: 16,
            bounces: 2 }
}

fn parse_materials(materials_json: &Json) -> TreeMap<String, Arc<Material>> {
    let mut material_map = TreeMap::new();
    let materials = materials_json.as_list()
                                  .expect("Materials isn't a list");
        
    for material in materials.iter() {
        let (name, mat) = parse_mat(material);
        material_map.insert(name, mat);
    }
        
    material_map
}

fn parse_mat(material_json: &Json) -> (String, Arc<Material>) {
    let name = material_json.find(&"name".to_string())
                            .expect("Material missing name")
                            .as_string()
                            .expect("Name is not a string");
    
    let color = material_json.find(&"color".to_string())
                             .expect("Material missing color")
                             .as_list()
                             .expect("Color not of format [r, g, b]");
    let r = color[0].as_f64().expect("Color should only contain numbers") as f32;
    let g = color[1].as_f64().expect("Color should only contain numbers") as f32;
    let b = color[2].as_f64().expect("Color should only contain numbers") as f32;
    
    let mat = Material { color: Color { r: r,
                                        g: g,
                                        b: b }
                         };
    (name.to_string(), Arc::new(mat))
}

fn parse_objects(objects_json: &Json, materials: &TreeMap<String, Arc<Material>>) -> Vec<SceneObject> {
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
