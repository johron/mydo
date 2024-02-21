use std::{path, process};
use execute::Execute;

use crate::util;

pub fn auto(mut args: Vec<String>) {
    if args.len().clone() == 0 {
        eprintln!("Error: No file was supplied");
        println!("Usage: 'do auto file'");
        process::exit(1)
    }
    
    let file = args[0].clone();
    if !path::Path::new(&file).exists() {
        eprintln!("Error: The file supplied does not exist: '{}'", file);
        println!("Usage: 'do auto 'file_path''");
        process::exit(1);
    }
    args.remove(0);
    
    let mut ext = "";
    let runner;
    let chunks: Vec<&str> = file.split(".").collect();
    
    for i in 0..chunks.len() {
        if i == chunks.len() - 1 {
            ext = chunks[i];
        }
    }
    
    let shebang_owned: String;
    
    match ext {
        "py" => runner = "/usr/bin/python3",
        "js" => runner = "/usr/bin/node",
        "c" => runner = "/usr/bin/cc",
        "cpp" => runner = "/usr/bin/c++",
        _ => {
            let shebang = util::get_shebang(&file).unwrap_or_else(|| {
                eprintln!("Error: No support for specified file extension: '{}'", ext);
                process::exit(1);
            });
            shebang_owned = shebang.to_owned();
            runner = &shebang_owned;
        }
    };
    
    println!("Job: Running '{}' with '{}'", &file, runner);
    let binding = process::Command::new(runner);
    let mut cmd = binding;
    cmd.args([file]);
    
    let output = cmd.execute_output().unwrap().stdout;
    
    if !&output.is_empty() {
        println!("{:?}", output);
    }
    
    println!("Job: Finished")
}

pub fn man(_args: Vec<String>) {
    println!("Todo: implement man");
}