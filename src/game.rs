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

pub fn parse() -> GameMap {
    let game_data = load();
    let mut locations: HashMap<String, String> = HashMap::new();

    let mut section = 1;
    for line in game_data.lines() {
        if line.starts_with("-1") {
            section = section + 1;
        }
        match section {
            1 => parse_location(line, &mut locations),
            _ => (),
        }
    }

    GameMap::new(locations)
}

fn load() -> String {
    read_to_string(Path::new("./src/advent.dat")).expect("Somethis whent wrong")
}

fn parse_location(line: &str, locations: &mut HashMap<String, String>) {
    let mut line_iter: Vec<String> = line
        .split_whitespace()
        .map(|s| s.trim_start().to_string())
        .collect();

    let position = line_iter.remove(0);
    let description = line_iter.join(" ");

    if locations.contains_key(&position) {
        let update_with = locations.get(&position).unwrap().to_owned();
        *locations.get_mut(&position).unwrap() = [update_with, description].join("\n")
    } else {
        locations.insert(position, description);
    }
}
