use std::{env};

pub fn get_home() -> String {
    env::var("HOME").unwrap_or_else(|_| String::from(""))
}