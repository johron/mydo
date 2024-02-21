mod cmd;
mod util;

extern crate config;
extern crate serde;
extern crate serde_derive;

use std::{collections, path::Path, env, process};
use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ExtensionConfig {
    bin: String,
    #[serde(default)]
    args: String,
}

#[derive(Debug, Deserialize)]
struct AppConfig {
    extention: std::collections::HashMap<String, ExtensionConfig>,
}

fn read_config_file() -> Result<AppConfig, config::ConfigError> {
    let mut config = Config::default();
    let config_path = dirs::home_dir()
        .unwrap_or_else(|| Path::new("/").to_path_buf())
        .join(".do-rs/Config.toml");
    config.merge(File::from(config_path))?;
    config.try_into()
}

fn get_extension_info(extension: &str) -> Option<ExtensionConfig> {
    if let Ok(app_config) = read_config_file() {
        if let Some(ext_config) = app_config.extention.get(extension) {
            return Some(ext_config.clone());
        }
    }
    None
}

fn main() {
    let config = config::Config::builder()
        .add_source(config::File::with_name("./.do-rs/Config"))
        .add_source(config::Environment::with_prefix("HOME"))
        .build()
        .unwrap();
    
    println!(
        "{:?}",
        config
            .try_deserialize::<collections::HashMap<String, String>>()
            .unwrap()
    );

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