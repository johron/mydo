use std::{path, process, time};
use execute::Execute;

use crate::util;

pub fn auto(parameters: &mut Vec<String>) {
    let presets = util::config::get_presets();
    let file = &parameters[0];
    
    if !path::Path::new(file).exists() {
        eprintln!("Error: Specified file does not exist: {}", file);
        process::exit(1);
    }
    
    let mut found = false;

    let shebang = util::shebang::get_shebang(&file).unwrap_or("None".parse().unwrap());
    let shebang_owned = shebang.to_owned();
    let runner = &shebang_owned;
    
    if shebang == "None" {
        for (key, preset) in presets.unwrap() {
            if key == "run" || key == "build" {}
            
            if file.ends_with(key.as_str()) {
                let runner = preset.as_str()
                    .expect("Expected a string")
                    .replace("{file}", file)
                    .replace("{home}", &util::home::get_home())
                    .replace("{output}", file.split(".")
                        .collect::<Vec<&str>>()[0]);
                
                let mut to_pass: Vec<&str> = runner.split(" ").collect();
                let binding = process::Command::new(to_pass[0]);
                let mut cmd = binding;
                
                to_pass.remove(0);
                cmd.args(to_pass);

                let now = time::SystemTime::now();
                cmd.execute_output().unwrap().stdout;

                let show_compilation_time = util::config::get_setting("show_time");
                if show_compilation_time.unwrap() == true {
                    println!("Time taken: {:.2}s", now.elapsed().unwrap().as_secs_f32());
                }

                found = true;
            }
        }

        if !found {
            eprintln!("Error: No preset found for '{}'", file);
            process::exit(1)
        }
    } else {
        let binding = process::Command::new(runner);
        let mut cmd = binding;
        cmd.args([file]);
        
        let now = time::SystemTime::now();
        cmd.execute_output().unwrap().stdout;

        let show_compilation_time = util::config::get_setting("show_time");
        if show_compilation_time.unwrap() == true {
            println!("Time taken: {:.2}s", now.elapsed().unwrap().as_secs_f32());
        }
    }
}
