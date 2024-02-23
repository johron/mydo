use std::{fs, process};
use serde_json::{Map, Value};

pub fn get_presets() -> Option<Map<String, Value>> {
    let binding = fs::read_to_string("/etc/do-rs/config.json")
        .unwrap_or_else(|err| panic!("Failed to read config file: {}", err));

    let config: Value = serde_json::from_str(&binding)
        .unwrap_or_else(|err| panic!("Failed to parse config JSON: {}", err));

    config["presets"].as_object().map(|obj| obj.clone())
}

pub fn get_setting(setting: &str) -> Option<Value> {
    let binding = fs::read_to_string("/etc/do-rs/config.json")
        .unwrap_or_else(|err| panic!("Failed to read config file: {}", err));

    let config: Value = serde_json::from_str(&binding)
        .unwrap_or_else(|err| panic!("Failed to parse config JSON: {}", err));

    if let Some(settings) = config["settings"].as_object() {
        if let Some(setting_value) = settings.get(setting) {
            return Some(setting_value.clone());
        } else {
            println!("Setting '{}' not found in config.json", setting);
        }
    } else {
        println!("'settings' object not found in config.json");
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