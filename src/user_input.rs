use std::io;

pub struct Input {
    pub verb: String,
    pub noun: String,
}

impl Input {
    fn new(verb: String, noun: String) -> Input {
        Input {
            verb: verb.to_uppercase(),
            noun: noun.to_uppercase(),
        }
    }
}

pub fn get() -> Input {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {}
        Err(_no_updates_is_fine) => {}
    }
    let inputs: Vec<String> = input
        .trim()
        .to_string()
        .split_whitespace()
        .map(str::to_string)
        .collect();

    match (inputs.get(0), inputs.get(1)) {
        (Some(verb), Some(noun)) => Input::new(verb.clone(), noun.clone()),
        (Some(verb), None) => Input::new(verb.clone(), "".to_string()),
        (None, _) => panic!("You must provide an input"),
    }
}
