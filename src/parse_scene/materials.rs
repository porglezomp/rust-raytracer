use scene::Material;
use std::collections::TreeMap;
use serialize::json::Json;
use std::sync::Arc;
use image_types::Color;

pub fn parse_materials(materials_json: &Json) -> TreeMap<String, Arc<Material>> {
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
