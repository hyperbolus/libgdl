use std::collections::HashMap;
use std::{env, fs};

struct Object {
    id: u32,    // 1
    x: f32,     // 2
    y: f32,     // 3
}

fn get_key_type(key: &str) -> Option<&'static str> {
    let type_lookup: HashMap<&str, &'static str> = [
        ("1", "u32"),
        ("2", "f32"),
        ("3", "f32"),
    ]
        .iter()
        .cloned()
        .collect();

    type_lookup.get(key).cloned()
}

enum Type {
    Int(i32),
    UnsignedInt(u32),
    Float(f32),
    String(String),
}

fn main() {
    println!("keyed2gml");
    // Read in file
    let data = fs::read_to_string(&env::args().nth(1).unwrap()).expect("Unable to read file");

    // Split into vector
    let objects_raw: Vec<&str> = data.split(';').collect();

    let mut objects: Vec<Object> = vec![];

    let mut skipped: u32 = 0;

    // First object (0) is special header
    let mut i = 1;
    'objects: while i < objects_raw.len() {
        if i % 10000 == 0 {
            println!("Object {}/{}", i / 2, objects_raw.len() / 2);
        }
        // Collect key:value pairs in a single object string
        let parts: Vec<&str> = objects_raw[i].split(',').collect();

        // HashMap for single object's keys and values
        let mut map: HashMap<u32, Type> = HashMap::new();

        let mut obj_index = 0;
        while obj_index < parts.len() {
            if parts[obj_index].is_empty() {
                i += 2;
                continue 'objects;
            }

            let key = parts[obj_index].to_string().parse::<u32>().unwrap();
            obj_index += 1;

            // If there is a value to the current key, we go by 2's remember
            if obj_index < parts.len() {
                let value = parts[obj_index].to_string();

                match get_key_type(&key.to_string()) {
                    Some("string") => {
                        map.insert(key, Type::String(value));
                    }
                    Some("string:b64") => {
                        //
                    }
                    Some("i32") => {
                        if let Ok(parsed_value) = value.parse::<i32>() {
                            map.insert(key, Type::Int(parsed_value));
                        }
                    }
                    Some("u32") => {
                        if let Ok(parsed_value) = value.parse::<u32>() {
                            map.insert(key, Type::UnsignedInt(parsed_value));
                        }
                    }
                    Some("f32") => {
                        if let Ok(parsed_value) = value.parse::<f32>() {
                            map.insert(key, Type::Float(parsed_value));
                        }
                    }
                    _ => {
                        skipped += 1;
                    }
                }

                // Done, move on to the next key
                obj_index += 1;
            }
        }

        objects.push(Object {
            id: match map.get(&1) {
                Some(&Type::UnsignedInt(x)) => x,
                _ => panic!("Error: x extraction failed.")
            },
            x: match map.get(&2) {
                Some(&Type::Float(x)) => x,
                _ => panic!("Error: x extraction failed.")
            },
            y: match map.get(&3) {
                Some(&Type::Float(x)) => x,
                _ => panic!("Error: x extraction failed.")
            },
        });

        i += 1;
    }

    let mut bytes: Vec<u8> = vec![];

    bytes.extend_from_slice("GDLVL".as_bytes());

    for object in objects {
        bytes.extend_from_slice(object.id.to_le_bytes().as_slice());
        bytes.extend_from_slice(object.x.to_le_bytes().as_slice());
        bytes.extend_from_slice(object.y.to_le_bytes().as_slice());
    }

    fs::write("out.gdl", bytes).expect("Oppsie poopsie");

    println!("Skipped keys: {}", skipped);
}
