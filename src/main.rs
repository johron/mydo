mod cmd;
mod util;

use std::{process, env};

fn show_help() {
    println!("do-rs (v{})", env!("CARGO_PKG_VERSION"));
    println!("> do (file)");
    println!("> do auto (file)");
    println!("  - will run this file through preset");
    println!("  - specified in config.json or if not");
    println!("  - present, run through shebang");
    println!("> do init (init)");
    println!("  - will initilaize a project from");
    println!("  - config.json, could be a local");
    println!("  - archive, init command, or");
    println!("  - network archive to be downloaded");
}

fn main() {
    let mut parameters: Vec<String> = env::args().collect();
    parameters.remove(0); // Remove the do-rs executable from arguments
    
    if parameters.len().clone() == 0 {
        show_help();
        process::exit(0);
    }
    
    match parameters[0].as_str() {
        "auto" => {
            parameters.remove(0);
            cmd::auto::auto(&mut parameters);
        },
        "init" => {
            parameters.remove(0);
            cmd::init::init(&parameters);
        },
        "help" => {
            show_help();
            process::exit(0);
        },
        _ => cmd::auto::auto(&mut parameters),
    }
}