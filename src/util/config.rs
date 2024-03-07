use std::{fs, process, path};
use serde_json::{Map, Value};
use crate::util;

fn get_path(project_root: bool) -> String {
    let home = util::home::get_home().to_string();

    let from_root = "mydo.json";
    let binding = home + "/.mydo/mydo.json";
    let from_home = binding.as_str();

    if project_root {
        if path::Path::new(from_root).exists() {
            return from_root.to_string();
        } else if path::Path::new(from_home).exists() {
            return from_home.to_string();
        } else {
            eprintln!("Error: Project config and home config file does not exist.");
            process::exit(1);
        }
    } else {
        if path::Path::new(from_home).exists() {
            return from_home.to_string();
        } else {
            eprintln!("Error: Home config file does not exist.");
            process::exit(1);
        }
    }
}

pub fn get_presets() -> Option<Map<String, Value>> {
    let path = get_path(true);
    let binding = fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("Failed to read config file: {}", err));

    let config: Value = serde_json::from_str(&binding)
        .unwrap_or_else(|err| panic!("Failed to parse config JSON: {}", err));

    config["presets"].as_object().map(|obj| obj.clone())
}

pub fn get_init(init: &str) -> Option<Value> {
    let path = get_path(false);
    let binding = fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("Failed to read config file: {}", err));

    let config: Value = serde_json::from_str(&binding)
        .unwrap_or_else(|err| panic!("Failed to parse config JSON: {}", err));

    if let Some(inits) = config["inits"].as_object() {
        if let Some(init_value) = inits.get(init) {
            return Some(init_value.clone());
        } else {
            eprintln!("Error: Init '{}' not found in mydo.json", init);
        }
    } else {
        eprintln!("Error: 'inits' object not found in mydo.json");
    }

    eprintln!("Error: No data in '{}' init", init);
    process::exit(1);
}

pub fn get_setting(setting: &str) -> Option<Value> {
    let path = get_path(false);
    let binding = fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("Failed to read config file: {}", err));

    let config: Value = serde_json::from_str(&binding)
        .unwrap_or_else(|err| panic!("Failed to parse config JSON: {}", err));

    if let Some(settings) = config["settings"].as_object() {
        if let Some(setting_value) = settings.get(setting) {
            return Some(setting_value.clone());
        } else {
            eprintln!("Error: Setting '{}' not found in mydo.json", setting);
        }
    } else {
        eprintln!("Error: 'settings' object not found in mydo.json");
    }

    eprintln!("Error: No data in '{}' setting", setting);
    process::exit(1);
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