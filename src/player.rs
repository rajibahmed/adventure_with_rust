pub fn new(name: String) -> Player {
    Player {
        name: name,
        initial_location: 0,
        current_location: 0,
        object: "stick".to_string(),
    }
}

pub struct Player {
    pub name: String,
    pub initial_location: u8,
    pub current_location: u8,
    pub object: String,
}

impl<'a> Player {
    pub fn say_hello(&self) {
        println!(
            "hello {} with location of {}",
            self.name, self.initial_location
        );
    }

    pub fn where_am_i(&self) {
        if self.object.is_empty() {
            println!("You are at {}", self.current_location);
        } else {
            println!("You are at {} with {}", self.current_location, self.object);
        }
    }
}
