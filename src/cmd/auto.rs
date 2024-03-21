use std::{path, process, time};
use execute::Execute;

use crate::util;

fn get_last(input: &str) -> Option<&str> {
    if let Some(index) = input.rfind('.') {
        // Add 1 to exclude the dot itself
        Some(&input[index + 1..])
    } else {
        None
    }
}

pub fn auto(parameters: &mut Vec<String>) {
    let file = &parameters[0];
    
    if !path::Path::new(file).exists() {
        eprintln!("Error: Specified file does not exist: {}", file);
        process::exit(1);
    }
    
    let shebang = util::shebang::get_shebang(&file).unwrap_or("None".parse().unwrap());
    let shebang_owned = shebang.to_owned();
    let runner = &shebang_owned;
    
    if shebang == "None" {
        let runner = util::config::get_runner(get_last(file).unwrap()).as_str()
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
        
        if let Ok(exec) = cmd.execute_output() {
            if !exec.status.success() {
                eprintln!("Error: Command execution failed");
                process::exit(1);
            }
            exec.stdout;
        } else {
            eprintln!("Error: Unable to execute command");
            process::exit(1);
        }

        let show_compilation_time = util::config::get_setting("show_time");
        if show_compilation_time.unwrap() == true {
            println!("Time taken: {:.2}s", now.elapsed().unwrap().as_secs_f32()); 
        }
    } else {
        let binding = process::Command::new(runner);
        let mut cmd = binding;
        cmd.args([file]);
        
        let now = time::SystemTime::now();
        
        if let Ok(exec) = cmd.execute_output() {
            if !exec.status.success() {
                eprintln!("Error: Command execution failed");
                process::exit(1);
            }
            exec.stdout;
        } else {
            eprintln!("Error: Unable to execute command");
            process::exit(1);
        }

        let show_compilation_time = util::config::get_setting("show_time");
        if show_compilation_time.unwrap() == true {
            println!("Time taken: {:.2}s", now.elapsed().unwrap().as_secs_f32());
        }
    }
}
