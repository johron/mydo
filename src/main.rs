mod cmd;
mod util;

use std::{process, env};

fn show_help() {
    println!("mydo (v{})", env!("CARGO_PKG_VERSION"));
    println!("\
    Usage: mydo [arg1] [arg2]
  > mydo [file_name]
  > mydo auto [file_name]
    - Run specified file with preset
      from ~/.mydo/mydo.json or run
      through shebang if not present.
  > mydo init [init_name]
    - Initialize a project from
      the ~/.mydo/mydo.json config
      file.
  > mydo help
   - Show this help message\
    ")
}

fn main() {
    let mut parameters: Vec<String> = env::args().collect();
    parameters.remove(0); // Remove the mydo executable from arguments
    
    if parameters.len().clone() == 0 {
        println!("mydo (v{})", env!("CARGO_PKG_VERSION"));
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