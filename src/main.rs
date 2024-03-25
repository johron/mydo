mod cmd;
mod util;

use std::{process, env};

fn show_help() {
    println!("mydo (v{})", env!("CARGO_PKG_VERSION"));
    println!("\
    Usage: mydo [args..]\
    Commands:
    auto    Run given file.
    init    Initialize project.
    run     Run project.
    build   Build project.

    --version  (-v)   Show mydo version.
    --help     (-h)   Display this help message.")
}

fn main() {
    let mut parameters: Vec<String> = env::args().collect();
    parameters.remove(0); // Remove the mydo executable from arguments
    
    if parameters.len().clone() == 0 {
        if util::config::get_setting("root").expect("Error: Setting 'root' is not a string?") == "run" {
            cmd::run::run(&mut parameters);
        } else if util::config::get_setting("root").expect("Error: Setting 'root' is not a string?") == "build" {
            cmd::build::build(&mut parameters);
        } else {
            println!("mydo (v{})", env!("CARGO_PKG_VERSION"));
            process::exit(0);
        }
    } else if parameters[0].as_str() == "auto" {
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
    } else if parameters[0].as_str() == "--help" || parameters[0].as_str() == "-h" {
        show_help();
        process::exit(0);
    } else if parameters[0].as_str() == "--version" || parameters[0].as_str() == "-v" {
        println!("mydo (v{})", env!("CARGO_PKG_VERSION"));
        process::exit(0);
    } else {
        cmd::auto::auto(&mut parameters);
    }
}