use scene::Scene;
use std::str;
use serialize::json;
use std::io::File;

mod lights;
mod objects;
mod materials;

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

    let contents = json_object.as_object()
        .expect("Top level of scene file isn't a JSON object");

    let scene_json = json_object.find(&"scene".to_string())
                                .expect("JSON missing scene section.")
                                .as_object()
                                .expect("JSON scene section is not an object");

    let num_gi_samples = scene_json.find(&"GI samples".to_string())
        .expect("Scene section missing entry 'GI samples'")
        .as_u64()
        .expect("'GI samples' was not a number") as u32;
    let num_shadow_samples = scene_json.find(&"shadow samples".to_string())
        .expect("Scene section missing entry 'shadow samples'")
        .as_u64()
        .expect("'shadow samples' was not a number") as u32;
    let mut num_bounces = scene_json.find(&"bounces".to_string())
        .expect("Scene section missing entry 'bounces'")
        .as_u64()
        .expect("'bounces' was not a number") as u32;

    if num_bounces > 4 {
        println!("Warning: {} bounces not supported, falling back to 4 bounces", num_bounces);
        num_bounces = 4;
    }

    let material_json = contents.find(&"materials".to_string())
        .expect("JSON missing materials section.");
    let materials = materials::parse_materials(material_json);

    let objects_json = contents.find(&"objects".to_string())
        .expect("JSON missing objects section.");
    let objects = objects::parse_objects(objects_json, &materials);

    let lights_json = contents.find(&"lights".to_string())
        .expect("JSON missing lights section");
    let lights = lights::parse_lights(lights_json);
        
    Scene { objects: objects,
            lights: lights,
            num_gi_samples: num_gi_samples,
            num_shadow_samples: num_shadow_samples,
            bounces: num_bounces }
}
