use std::{process, time};
use execute::Execute;
use serde_json::{Value};

use crate::util;

pub fn auto(parameters: &mut Vec<String>) {
    let presets = util::config::get_presets();
    let file = &parameters[0];
    let mut found = false;

    let shebang = util::shebang::get_shebang(&file).unwrap_or("None".parse().unwrap());
    let shebang_owned = shebang.to_owned();
    let runner = &shebang_owned;
    
    if shebang == "None" {
        for (preset, args) in presets.unwrap() {
            if file.ends_with(preset.as_str()) {
                if args.get(1) == Some(&Value::Null) {
                    eprintln!("Error: No argument for the file to run in config.json");
                    println!("Fix: See documentation when that is a thing");
                    process::exit(1);
                }

                let runner = &args[0].as_str().expect("Expected a string");
                let binding = process::Command::new(runner);
                let mut cmd = binding;
                cmd.args([file]);

                let now = time::SystemTime::now();
                cmd.execute_output().unwrap().stdout;

                let show_compilation_time = util::config::get_setting("show_compilation_time");
                if show_compilation_time.unwrap() == true {
                    println!("Time taken: {}ms", now.elapsed().unwrap().as_millis());
                }

                found = true
            }
        }

        if !found {
            println!("Error: No preset found or file not found: '{}'", file);
            process::exit(1)
        }
    } else {
        let binding = process::Command::new(runner);
        let mut cmd = binding;
        cmd.args([file]);
        
        let now = time::SystemTime::now();
        cmd.execute_output().unwrap().stdout;

        let show_compilation_time = util::config::get_setting("show_compilation_time");
        if show_compilation_time.unwrap() == true {
            println!("Time taken: {}ms", now.elapsed().unwrap().as_millis());
        }
    }
}
