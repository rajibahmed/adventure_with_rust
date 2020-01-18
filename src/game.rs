use std::collections::HashMap;
use std::{fs::read_to_string, path::Path};

#[allow(dead_code)]
struct Node {}

#[allow(dead_code)]
struct Element {}

pub fn load(filename: String) -> String {
    let path = Path::new(&filename);
    read_to_string(path).expect("Somethis whent wrong")
}

fn parse_location(line: String) -> (String, String) {
    let mut line_iter: Vec<String> = line
        .split_whitespace()
        .map(|s| s.trim().to_string())
        .collect();
    (line_iter.remove(0), line_iter.join(" "))
}

pub fn parse() -> String {
    let game_data = load("./src/advent.dat".to_string());
    let mut locations: HashMap<String, String> = HashMap::new();

    let mut section = 1;
    for line in game_data.lines() {
        if line.starts_with("-1") {
            section = section + 1;
        }
        match section {
            1 => {
                let location = parse_location(line.to_string());
                if locations.contains_key(&location.0) {
                    let update_with = locations.get(&location.0).unwrap().to_owned();
                    *locations.get_mut(&location.0).unwrap() = [update_with, location.1].join(" ");
                } else {
                    locations.insert(location.0, location.1);
                }
            }
            2 => (),
            _ => (),
        }
    }
    println!("{:?}", locations.get("1").unwrap());

    "some string".to_string()
}
