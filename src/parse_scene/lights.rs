use scene::{SceneLight, DirectionalLight, PointLight, Illuminator};
use serialize::json::{Json, JsonObject};
use image_types::Color;
use cgmath::{Point3, Vector3, EuclideanVector};

pub fn parse_lights(lights_json: &Json) -> Vec<SceneLight> {
    let lights = lights_json.as_list()
                            .expect("Lights ins't a list");
    let mut scene_lights = Vec::with_capacity(lights.len());
    for light in lights.iter() {
        let lght = parse_light(light);
        scene_lights.push(lght);
    }
    scene_lights
}

fn parse_light(light_json: &Json) -> SceneLight {
    let light = light_json.as_object()
                          .expect("Light isn't a JSON object");

    let light_type = light.find(&"type".to_string())
                          .expect("Light doesn't have a type")
                          .as_string()
                          .expect("Light type isn't a string");
    let light_object = match light_type.as_slice() {
        "directional light" => directional_from_json(light),
        "point light"       => point_from_json(light),
        x                   => fail!("Unsupported light type '{}'", x)
    };
    
    SceneLight { illuminator: light_object }
}


fn point_from_json(light: &JsonObject) -> Box<Illuminator+Send+Sync> {
    let pos = light.find(&"position".to_string())
        .expect("Light doesn't have a position")
        .as_list()
        .expect("Light position isn't of form [x, y, z]");
    let color = light.find(&"color".to_string())
        .expect("Light doesn't have a color")
        .as_list()
        .expect("Light color isn't of form [r, g, b]");
    let intensity = light.find(&"intensity".to_string())
        .expect("Light doesn't have intensity")
        .as_f64()
        .expect("Light intensity isn't a number") as f32;
    let x = pos[0].as_f64().expect("Position should only contain numbers") as f32;
    let y = pos[1].as_f64().expect("Position should only contain numbers") as f32;
    let z = pos[2].as_f64().expect("Position should only contain numbers") as f32;
    let r = color[0].as_f64().expect("Color should only contain numbers") as f32;
    let g = color[1].as_f64().expect("Color should only contain numbers") as f32;
    let b = color[2].as_f64().expect("Color should only contain numbers") as f32;
        
    let radius = light.find(&"radius".to_string())
        .expect("Point light doesn't have radius")
        .as_f64()
        .expect("Point light radius isn't a number") as f32;
    
    box PointLight { position: Point3::new(x, y, z),
                     color: Color { r: r, g: g, b: b },
                     intensity: intensity,
                     radius: radius }
}



fn directional_from_json(light: &JsonObject) -> Box<Illuminator+Send+Sync> {
    let pos = light.find(&"direction".to_string())
        .expect("Light doesn't have a direction")
        .as_list()
        .expect("Light position isn't of form [x, y, z]");
    let color = light.find(&"color".to_string())
        .expect("Light doesn't have a color")
        .as_list()
        .expect("Light color isn't of form [r, g, b]");
    let intensity = light.find(&"intensity".to_string())
        .expect("Light doesn't have intensity")
        .as_f64()
        .expect("Light intensity isn't a number") as f32;
    let x = pos[0].as_f64().expect("Direction should only contain numbers") as f32;
    let y = pos[1].as_f64().expect("Direction should only contain numbers") as f32;
    let z = pos[2].as_f64().expect("Direction should only contain numbers") as f32;
    let r = color[0].as_f64().expect("Color should only contain numbers") as f32;
    let g = color[1].as_f64().expect("Color should only contain numbers") as f32;
    let b = color[2].as_f64().expect("Color should only contain numbers") as f32;
    
    let angle = light.find(&"angle".to_string())
        .expect("Directional light doesn't have angle")
        .as_f64()
        .expect("Directional light angle isn't a number") as f32;
            

    box DirectionalLight { direction: Vector3::new(x, y, z).normalize(),
                           color: Color { r: r, g: g, b: b },
                           intensity: intensity,
                           angle: angle }
}
