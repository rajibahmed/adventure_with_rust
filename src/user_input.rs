use std::io;

pub enum Direction {
    UP,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::UP
    }
}

#[derive(Default)]
pub struct Input {
    pub verb: String,
    pub direction: Direction,
}

impl Input {
    fn new(verb: String) -> Input {
        Input {
            verb: verb.to_uppercase(),
            ..Default::default()
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

    Input::new(inputs.remove(0))
}
