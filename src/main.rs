mod cmd;
mod util;

use std::{fs, path, io::Write, env, process};
use config;

fn main() {
    // check if ~/.do-rs/config.toml exists else create that
    let config_path = "~/.do-rs/config.toml";
    if !path::Path::new(config_path).exists() {
        let mut config_file = fs::File::create(config_path)?;
        config_file.write_all(b"")?;
        
    }

    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    // dbg!(&args);
    
    // wrk auto script.py // will automatically choose runner by file extension or shebang
    // wrk man /usr/bin/python3 script.py // will pass file to specified binary
    
    if args.len().clone() == 0 {
        println!("do-rs version 0.1.0");
        process::exit(0);
    }
    
    match args[0].as_str() {
        "auto" => {
            args.remove(0);
            cmd::auto(args);
        },
        "man" => {
            args.remove(0);
            cmd::man(args);
        },
        _=> {
            eprintln!("Error: Could not recognize the command: '{}'", args[0]);
            process::exit(1);
        },
    }
}