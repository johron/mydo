use std::{fs, process};
use serde_json::{Value};
use crate::util;

fn read_main_config() -> Value {
    let path = util::home::get_home().to_string() + "/.mydo/mydo.json";

    let binding = fs::read_to_string(path)
        .unwrap_or_else(|err| {
            eprintln!("Failed to read config JSON: {}", err);
            process::exit(1)
        });

    let config: Value = serde_json::from_str(&binding)
        .unwrap_or_else(|err| {
            eprintln!("Failed to parse config JSON: {}", err);
            process::exit(1)
        });
    
    config
}

fn read_project_config() -> Value {
    let path = "mydo.json";

    let binding = fs::read_to_string(path)
        .unwrap_or_else(|err| {
            eprintln!("Failed to read config JSON: {}", err);
            process::exit(1)
        });

    let config: Value = serde_json::from_str(&binding)
        .unwrap_or_else(|err| {
            eprintln!("Failed to parse config JSON: {}", err);
            process::exit(1)
        });

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

    process::exit(1);
}

pub fn get_runner(runner: &str) -> Value {
    let config = read_project_config();

    let runners = config["runners"].as_object().map(|obj| obj.clone()).unwrap();
    
   for (p, args) in runners {
       if p == runner {
           return args;
       }
   }

    eprintln!("Error: Requested preset '{runner}' does not exist");
    process::exit(1);
}

pub fn get_run() -> Option<Value> {
    let config = read_project_config();

    if let Some(setting_value) = config.get("run") {
        return Some(setting_value.clone());
    } else {
        eprintln!("Error: Run field not found in ~/.mydo/mydo.json");
    }
    process::exit(1);
}

pub fn get_build() -> Option<Value> {
    let config = read_project_config();

    if let Some(setting_value) = config.get("build") {
        return Some(setting_value.clone());
    } else {
        eprintln!("Error: Build field not found in ~/.mydo/mydo.json");
    }
    process::exit(1);
}