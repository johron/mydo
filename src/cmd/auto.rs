use std::{process};
use serde_json::{Value};

use crate::util;

pub fn auto(parameters: &mut Vec<String>) {
    parameters.remove(0);
    let presets = util::config::get_presets();
    let file = &parameters[0];
    
    for (preset, args) in presets.unwrap() {
        if file.ends_with(preset.as_str()) {
            if &args[1] == &Value::Null {
                eprintln!("Error: No argument for the file to run in config.json");
                println!("Fix: See documentation when that is a thing");
                process::exit(1);
            }
            
            let runner = &args[0];

            
            println!("Now: Running '{file}' with '{runner}' from the '{preset}' preset.");
        }
    }
}