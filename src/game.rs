use std::{fs::read_to_string, path::Path};

pub fn load(filename: String) -> String {
    let path = Path::new(&filename);
    read_to_string(path).expect("Somethis whent wrong")
}

pub fn parse() -> String {
    load("./src/advent.dat".to_string())
}
