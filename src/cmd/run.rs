use std::{process, time};
use execute::Execute;
use crate::util;

pub fn run(args: &mut Vec<String>) {
    let preset = util::config::get_run();

    let runner = preset.unwrap().as_str()
        .expect("Expected a string")
        .replace("{home}", &util::home::get_home());

    let mut to_pass: Vec<&str> = runner.split(" ").collect();
    let binding = process::Command::new(to_pass[0]);
    let mut cmd = binding;

    to_pass.remove(0);
    cmd.args(to_pass);
    cmd.args(args);

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

    let show_compilation_time = util::config::get_setting("show_time", None);
    if show_compilation_time.unwrap() == true {
        println!("Time taken: {:.2}s", now.elapsed().unwrap().as_secs_f32());
    }
}