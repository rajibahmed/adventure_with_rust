pub fn new(name: String) -> Player {
    Player {
        name: name,
        initial_location: "1".to_string(),
        current_location: "1".to_string(),
        object: "stick".to_string(),
    }
}

pub struct Player {
    pub name: String,
    pub initial_location: String,
    pub current_location: String,
    pub object: String,
}

#[allow(dead_code)]
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
