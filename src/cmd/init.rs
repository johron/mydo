use std::{process, fs};
use std::io::{Write};
use execute::Execute;
use curl::easy;

use crate::util;

pub fn init(args: &Vec<String>) {
    if args.len() == 0 {
        eprintln!("Error: No init specified");
        process::exit(1);
    }
    
    let init = &args[0];
    let binding = util::config::get_init(init).unwrap().to_string();
    let init_path = binding
        .replace("{home}", &util::home::get_home());
    
    let binding = init_path.replace("\"", "");
    let mut to_pass: Vec<&str> = binding.split(" ").collect();

    if to_pass[0].starts_with("https://") || args[0].starts_with("http://") { // this is an init package to download
        let mut dst = Vec::new();
        let mut easy = easy::Easy::new();
        easy.url(to_pass[0]).unwrap();
        let _redirect = easy.follow_location(true);
        {
            let mut transfer = easy.transfer();
            transfer.write_function(|data| {
                dst.extend_from_slice(data);
                Ok(data.len())
            }).unwrap();
            transfer.perform().unwrap();
        }
        {
            let file = fs::File::create("./init.tar");
            file.expect("Error: download error").write_all(dst.as_slice()).expect("Error: different donwload error");
        }

        let mut tar = tar::Archive::new(fs::File::open("./init.tar").unwrap());
        tar.unpack(".").unwrap();
        let _ = fs::remove_file("./init.tar");
    } else if to_pass[0].ends_with(".tar") { // local init package to copy and decompress
        let _ = fs::copy(to_pass[0], "./init.tar");
        let mut tar = tar::Archive::new(fs::File::open("./init.tar").unwrap());
        tar.unpack(".").unwrap();
        let _ = fs::remove_file("./init.tar");
    } else { // run command to init
        let binding = process::Command::new(to_pass[0]);
        let mut cmd = binding;
        
        to_pass.remove(0);
        cmd.args(to_pass.clone());
        cmd.execute_output().unwrap().stdout;
    }

    let conf = r#"{
    "presets": {}
}
"#;
    
    fs::write("mydo.json", conf).expect("Error: Something happened while writing the mydo.json file");
}