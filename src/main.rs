mod cmd;
mod util;

use std::{process, env};

fn show_help() {
    println!("mydo (v{})", env!("CARGO_PKG_VERSION"));
    println!("> mydodo (file)");
    println!("> mydodo auto (file)");
    println!("  - will run this file through preset");
    println!("  - specified in config.json or if not");
    println!("  - present, run through shebang");
    println!("> mydo init (init)");
    println!("  - will initilaize a project from");
    println!("  - config.json, could be a local");
    println!("  - archive, init command, or");
    println!("  - network archive to be downloaded");
}

fn main() {
    let mut parameters: Vec<String> = env::args().collect();
    parameters.remove(0); // Remove the mydo executable from arguments
    
    if parameters.len().clone() == 0 {
        show_help();
        process::exit(0);
    }

    if parameters[0].as_str() == "auto" {
        parameters.remove(0);
        cmd::auto::auto(&mut parameters);
    } else if parameters[0].as_str() == "init" {
        parameters.remove(0);
        cmd::init::init(&parameters);
    } else if parameters[0].as_str() == "run" {
        parameters.remove(0);
        cmd::run::run(&mut  parameters);
    } else if parameters[0].as_str() == "build" {
        parameters.remove(0);
        cmd::build::build(&mut parameters);
    } else if parameters[0].as_str() == "help" {
        show_help();
        process::exit(0);
    } else {
        cmd::auto::auto(&mut parameters);
    }
}