use std::{fs, process, path, env};
use serde_json::{Map, Value};

fn get_path() -> String {
    let home = env::var("HOME").unwrap().to_string();
    
    let from_root = "do.json";
    let binding = home + "/.do-rs/do.json";
    let from_home = binding.as_str();
    
    if path::Path::new(from_root).exists() {
        return from_root.to_string();
    } else if path::Path::new(from_home).exists() {
        return from_home.to_string();
    } else {
        eprintln!("Error: Config file does not exist.");
        process::exit(1);
    }
}

pub fn get_presets() -> Option<Map<String, Value>> {
    let path = get_path();
    let binding = fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("Failed to read config file: {}", err));

    let config: Value = serde_json::from_str(&binding)
        .unwrap_or_else(|err| panic!("Failed to parse config JSON: {}", err));

    config["presets"].as_object().map(|obj| obj.clone())
}

pub fn get_setting(setting: &str) -> Option<Value> {
    let path = get_path();
    let binding = fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("Failed to read config file: {}", err));

    let config: Value = serde_json::from_str(&binding)
        .unwrap_or_else(|err| panic!("Failed to parse config JSON: {}", err));

    if let Some(settings) = config["settings"].as_object() {
        if let Some(setting_value) = settings.get(setting) {
            return Some(setting_value.clone());
        } else {
            println!("Setting '{}' not found in do.json", setting);
        }
    } else {
        println!("'settings' object not found in do.json");
    }

    println!("Error: No data in '{}' setting", setting);
    process::exit(1);
}

/*pub fn get_preset(preset: &str) -> Value {
    let presets = get_presets().unwrap();
    
    for (p, args) in presets {
        if p == preset {
            return args;
        }
    }
    
    eprintln!("Error: Requested preset '{preset}' does not exist");
    process::exit(1);
}*/