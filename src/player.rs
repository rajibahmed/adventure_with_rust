pub struct Player {
    pub name: String,
    pub location: String,
    pub object: String,
}

impl Player {
    pub fn new(name: &str) -> Player {
        Player {
            name: name.to_string(),
            ..Default::default()
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Player {
            name: "".to_string(),
            location: "1".to_string(),
            object: "".to_string(),
        }
    }
}

#[allow(dead_code)]
impl<'a> Player {
    pub fn say_hello(&self) {
        println!("hello {} with location of {}", self.name, self.location);
    }

    pub fn where_am_i(&self) {
        if self.object.is_empty() {
            println!("You are at {}", self.location);
        } else {
            println!("You are at {} with {}", self.location, self.object);
        }
    }
}
