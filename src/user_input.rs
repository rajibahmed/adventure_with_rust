use std::io;

pub enum Direction {
    UP,
}

pub struct Input {
    pub verb: String,
    pub direction: Direction,
}

impl Input {
    fn new(verb: String, direction: Direction) -> Input {
        Input {
            verb: verb,
            direction: direction,
        }
    }
}

pub fn get() -> Input {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {}
        Err(_no_updates_is_fine) => {}
    }
    let mut inputs: Vec<String> = input
        .trim()
        .to_string()
        .split_whitespace()
        .map(str::to_string)
        .collect();

    Input::new(inputs.remove(0), Direction::UP)
}
