use std::{fs, process};
use serde_json::{Map, Value};

pub fn get_presets() -> Option<Map<String, Value>> {
    let binding = fs::read_to_string("config.json")
        .unwrap_or_else(|err| panic!("Failed to read config file: {}", err));

    let config: Value = serde_json::from_str(&binding)
        .unwrap_or_else(|err| panic!("Failed to parse config JSON: {}", err));

    config["presets"].as_object().map(|obj| obj.clone())
}

pub fn get_preset(preset: &str) -> Value {
    let presets = get_presets().unwrap();
    
    for (p, args) in presets {
        if p == preset {
            return args;
        }
    }
    
    eprintln!("Error: Requested preset '{preset}' does not exist");
    process::exit(1);
}