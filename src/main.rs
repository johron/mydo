mod cmd;
mod util;

use std::{process, env};

fn main() {
    let mut parameters: Vec<String> = env::args().collect();
    parameters.remove(0); // Remove the do-rs executable from arguments
    
    if parameters.len().clone() == 0 {
        eprintln!("do-rs cli v1.0");
        process::exit(1);
    }
    
    match parameters[0].as_str() {
        "auto" => {
            parameters.remove(0);
            cmd::auto::auto(&mut parameters);
        },
        // "man" => {
        //     parameters.remove(0);
        //     cmd::man::man(&parameters);
        // },
        _ => cmd::auto::auto(&mut parameters),
    }
}