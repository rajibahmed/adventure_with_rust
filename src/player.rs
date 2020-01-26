#[derive(Debug, PartialEq)]
pub struct Player {
    pub name: String,
    pub location: String,
    pub verb: String,
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
            verb: "".to_string(),
        }
    }
}
