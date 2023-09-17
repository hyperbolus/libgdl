use std::{env, fs};
use std::time::Instant;

enum Value {
    UnsignedInt8(),
    UnsignedInt16(),
    UnsignedInt32(),
    SignedInt32(),
    Bool(),
    Float(),
    String(),
    StringB64(),
    None(),
}

fn main() {
    let start = Instant::now();
    let mut skipped: u32 = 0;
    let mut bytes: Vec<u8> = vec![];
    bytes.extend_from_slice("GDLVL".as_bytes());
    println!("Working...");

    // <editor-fold Lookup Table>
    let type_lookup: [Value; 109] = [
        Value::None(), // key starts at 1, this will never be used
        Value::UnsignedInt16(),
        Value::Float(),
        Value::Float(),
        Value::Bool(),
        Value::Bool(),
        Value::Float(),
        Value::UnsignedInt8(),
        Value::UnsignedInt8(),
        Value::UnsignedInt8(),
        Value::Float(),
        Value::Bool(),
        Value::UnsignedInt8(),
        Value::Bool(),
        Value::Bool(),
        Value::Bool(),
        Value::Bool(),
        Value::Bool(),
        Value::None(),
        Value::UnsignedInt16(),
        Value::SignedInt32(),
        Value::UnsignedInt32(),
        Value::UnsignedInt16(),
        Value::UnsignedInt16(),
        Value::SignedInt32(),
        Value::SignedInt32(),
        Value::None(),
        Value::None(),
        Value::Float(),
        Value::Float(),
        Value::None(), //"easing",
        Value::StringB64(), //"string:b64",
        Value::Float(),
        Value::UnsignedInt16(),
        Value::Bool(),
        Value::Float(),
        Value::None(),
        Value::None(),
        Value::None(),
        Value::None(),
        Value::None(),
        Value::Bool(),
        Value::Bool(),
        Value::None(), //"hsv",
        Value::None(), //"hsv",
        Value::Float(),
        Value::Float(),
        Value::Float(),
        Value::None(), //"pulse_mode",
        Value::None(), //"hsv",
        Value::UnsignedInt16(),
        Value::UnsignedInt16(),
        Value::None(), //"pulse_type",
        Value::None(),
        Value::Float(),
        Value::Bool(),
        Value::Bool(),
        Value::None(), //"integer_array",
        Value::Bool(),
        Value::Bool(),
        Value::Bool(),
        Value::SignedInt32(),
        Value::Bool(),
        Value::Float(),
        Value::Bool(),
        Value::Bool(),
        Value::Bool(),
        Value::Bool(),
        Value::SignedInt32(),
        Value::SignedInt32(),
        Value::Bool(),
        Value::UnsignedInt16(),
        Value::Float(),
        Value::None(),
        Value::Float(),
        Value::Float(),
        Value::UnsignedInt16(),
        Value::SignedInt32(),
        Value::Bool(),
        Value::None(), //"pickup_item_mode",
        Value::UnsignedInt32(),
        Value::Bool(),
        Value::None(), //"touch_toggle_mode",
        Value::None(),
        Value::Float(),
        Value::Float(),
        Value::Bool(),
        Value::Bool(),
        Value::None(), //"instant_count_comparison",
        Value::Bool(),
        Value::Float(),
        Value::Float(),
        Value::Float(),
        Value::Bool(),
        Value::Bool(),
        Value::UnsignedInt16(),
        Value::Bool(),
        Value::Float(),
        Value::Bool(),
        Value::Bool(),
        Value::Bool(),
        Value::None(), //"target_pos_coordinates",
        Value::Bool(),
        Value::Bool(),
        Value::Bool(),
        Value::Float(),
        Value::Bool(),
        Value::Float(),
        Value::UnsignedInt16(),
    ];
    // </editor-fold>

    let data = fs::read_to_string(&env::args().nth(1).unwrap()).expect("Unable to read file");
    let objects_raw: Vec<&str> = data.split(';').collect();

    // First object (0) is special header
    let mut i = 1;
    'objects: while i < objects_raw.len() {
        // Collect key:value pairs in a single object string
        let parts: Vec<&str> = objects_raw[i].split(',').collect();

        let mut obj_index = 0;
        while obj_index < parts.len() {
            if parts[obj_index].is_empty() {
                i += 2;
                continue 'objects;
            }

            let key = parts[obj_index].parse::<u8>().unwrap();
            obj_index += 1;

            // If there is a value to the current key, we go by 2's remember
            if obj_index < parts.len() {
                let value = parts[obj_index].to_string();

                match type_lookup[key as usize] {
                    Value::String() => bytes.extend_from_slice(value.as_bytes()),
                    Value::StringB64() => bytes.extend_from_slice(value.as_bytes()),
                    Value::SignedInt32() => {
                        if let Ok(parsed_value) = value.parse::<i32>() {
                            bytes.extend_from_slice(parsed_value.to_le_bytes().as_slice());
                        }
                    }
                    Value::UnsignedInt32() => {
                        if let Ok(parsed_value) = value.parse::<u32>() {
                            bytes.extend_from_slice(parsed_value.to_le_bytes().as_slice());
                        }
                    }
                    Value::UnsignedInt16() => {
                        if let Ok(parsed_value) = value.parse::<u16>() {
                            bytes.extend_from_slice(parsed_value.to_le_bytes().as_slice());
                        }
                    }
                    Value::UnsignedInt8() => {
                        if let Ok(parsed_value) = value.parse::<u8>() {
                            bytes.extend_from_slice(parsed_value.to_le_bytes().as_slice());
                        }
                    }
                    Value::Float() => {
                        if let Ok(parsed_value) = value.parse::<f32>() {
                            bytes.extend_from_slice(parsed_value.to_le_bytes().as_slice());
                        }
                    }
                    Value::Bool() => {
                        if let Ok(parsed_value) = value.parse::<bool>() {
                            if parsed_value {
                                bytes.push(0x01);
                            } else {
                                bytes.push(0x00);
                            }
                        }
                    }
                    _ => skipped += 1
                }

                // Done, move on to the next key
                obj_index += 1;
            }
        }

        i += 1;
    }

    println!("Done in {:?}. Skipped {} keys. Writing file...", start.elapsed(), skipped);
    fs::write("out.gdl", bytes).expect("Oppsie poopsie");
}
