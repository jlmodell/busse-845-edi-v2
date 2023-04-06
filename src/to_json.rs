use std::fs::File;
use std::io::prelude::*;

pub fn to_json<T>(payload: &T) where T: serde::Serialize {
    let mut file = File::create("output.json").unwrap();
    let json = serde_json::to_string_pretty(payload).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

pub fn combine_schema_with_output_to_json() {
    let mut schema_file = File::open("schema.json").unwrap();
    let mut schema = String::new();
    schema_file.read_to_string(&mut schema).unwrap();

    let mut schema_json: serde_json::Value = serde_json::from_str(&schema).unwrap();

    let mut output_file = File::open("output.json").unwrap();
    let mut output = String::new();
    output_file.read_to_string(&mut output).unwrap();

    let output_json: serde_json::Value = serde_json::from_str(&output).unwrap();

    schema_json["default"] = output_json["default"].clone();

    let output_str = serde_json::to_string_pretty(&schema_json).unwrap();

    let mut combined_file = File::create("combined.json").unwrap();
    combined_file.write_all(output_str.as_bytes()).unwrap();
}