use std::collections::HashMap;
use std::{fs::read_to_string, path::Path};

#[allow(dead_code)]
struct Node {}

#[allow(dead_code)]
struct Element {}

pub struct GameMap {
    pub locations: HashMap<String, String>,
}

impl GameMap {
    fn new(locations: HashMap<String, String>) -> GameMap {
        GameMap {
            locations: locations,
        }
    }
}

pub fn load(filename: String) -> String {
    let path = Path::new(&filename);
    read_to_string(path).expect("Somethis whent wrong")
}

fn parse_location(line: String, locations: &mut HashMap<String, String>) {
    let mut line_iter: Vec<String> = line
        .split_whitespace()
        .map(|s| s.trim().to_string())
        .collect();

    let position = line_iter.remove(0);
    let description = line_iter.join(" ");

    if locations.contains_key(&position) {
        let update_with = locations.get(&position).unwrap().to_owned();
        *locations.get_mut(&position).unwrap() = [update_with, description].join(" ")
    } else {
        locations.insert(position, description);
    }
}

pub fn parse() -> GameMap {
    let game_data = load("./src/advent.dat".to_string());
    let mut locations: HashMap<String, String> = HashMap::new();

    let mut section = 1;
    for line in game_data.lines() {
        if line.starts_with("-1") {
            section = section + 1;
        }
        match section {
            1 => parse_location(line.to_string(), &mut locations),
            _ => (),
        }
    }

    GameMap::new(locations)
}
