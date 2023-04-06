use std::fs::File;
use std::io::prelude::*;

const OUTPUT_FILE: &str = "output.json";
const SCHEMA_FILE: &str = "schema.json";
const COMBINED_FILE: &str = "combined.json";

const DEFAULT_SCHEMA_KEY: &str = "default";

pub fn to_json<T>(payload: &T) -> Result<(), Box<dyn std::error::Error>> where T: serde::Serialize {
    let mut file = File::create(OUTPUT_FILE)?;
    let json = serde_json::to_string_pretty(payload)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}

pub fn combine_schema_with_output_to_json() -> Result<(), Box<dyn std::error::Error>> {
    let mut schema_file = File::open(SCHEMA_FILE)?;
    let mut schema = String::new();
    schema_file.read_to_string(&mut schema)?;

    let mut schema_json: serde_json::Value = serde_json::from_str(&schema)?;

    let mut output_file = File::open(OUTPUT_FILE)?;
    let mut output = String::new();
    output_file.read_to_string(&mut output)?;

    let output_json: serde_json::Value = serde_json::from_str(&output)?;

    schema_json[DEFAULT_SCHEMA_KEY] = output_json[DEFAULT_SCHEMA_KEY].clone();

    let output_str = serde_json::to_string_pretty(&schema_json)?;

    let mut combined_file = File::create(COMBINED_FILE)?;
    combined_file.write_all(output_str.as_bytes())?;

    Ok(())
}