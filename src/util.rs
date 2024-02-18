use std::{fs, io::{self, BufRead}};

pub fn get_shebang(file_path: &str) -> Option<String> {
    if let Ok(file) = fs::File::open(file_path) {
        let reader = io::BufReader::new(file);
        if let Some(Ok(first_line)) = reader.lines().next() {
            if first_line.starts_with("#!") {
                let shebang: String = first_line.chars().skip(2).collect();
                return Some(shebang);
            }
        }
    }
    None
}