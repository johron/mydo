mod cmd;
mod util;

use std::{process, env, path};

fn main() {
    let mut parameters: Vec<String> = env::args().collect();
    parameters.remove(0); // Remove the do-rs executable from arguments
    
    if parameters.len().clone() == 0 {
        eprintln!("do-rs cli unspecified development version");
        process::exit(1);
    }
    
    if !path::Path::new("config.json").exists() {
        eprintln!("Error: Config file does not exist.");
        process::exit(1);
    }
    
    match parameters[0].as_str() {
        "auto" => cmd::auto::auto(&mut parameters),
        "man" => cmd::man::man(&parameters),
        _ => {
            eprintln!("Error: Unrecognized command: {}", parameters[0]);
            process::exit(1);
        }
    }
}