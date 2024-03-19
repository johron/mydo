use std::{fs, process};
use serde_json::{Value};
use crate::util;

fn read_main_config() -> Value {
    let path = util::home::get_home().to_string() + "/.mydo/mydo.json";
    
    let binding = fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("Failed to read config file: {}", err));

    let config: Value = serde_json::from_str(&binding)
        .unwrap_or_else(|err| panic!("Failed to parse config JSON: {}", err));
    
    config
}

fn read_project_config() -> Value {
    let path = "mydo.json";

    let binding = fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("Failed to read config file: {}", err));

    let config: Value = serde_json::from_str(&binding)
        .unwrap_or_else(|err| panic!("Failed to parse config JSON: {}", err));

    config
}

pub fn get_setting(setting: &str) -> Option<Value> {
    let config = read_main_config();

    if let Some(settings) = config["settings"].as_object() {
        if let Some(setting_value) = settings.get(setting) {
            return Some(setting_value.clone());
        } else {
            eprintln!("Error: Setting '{}' not found in ~/.mydo/mydo.json", setting);
        }
    } else {
        eprintln!("Error: 'settings' object not found in ~/.mydo/mydo.json");
    }

    eprintln!("Error: No data in '{}' setting", setting);
    process::exit(1);
}

pub fn get_init(init: &str) -> Option<Value> {
    let config = read_main_config();

    if let Some(settings) = config["inits"].as_object() {
        if let Some(setting_value) = settings.get(init) {
            return Some(setting_value.clone());
        } else {
            eprintln!("Error: Init '{}' not found in ~/.mydo/mydo.json", init);
        }
    } else {
        eprintln!("Error: 'inits' object not found in ~/.mydo/mydo.json");
    }

    eprintln!("Error: No data in '{}' init", init);
    process::exit(1);
}

pub fn get_preset(preset: &str) -> Value {
    let config = read_project_config();

    let presets = config["presets"].as_object().map(|obj| obj.clone()).unwrap();
    
   for (p, args) in presets {
       if p == preset {
           return args;
       }
   }

    eprintln!("Error: Requested preset '{preset}' does not exist");
    process::exit(1);
}
