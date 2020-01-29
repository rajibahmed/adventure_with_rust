use game::Node;
#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub name: String,
    pub location: String,
    pub verb: String,
}

impl Player {
    pub fn new(name: &str) -> Self {
        Player {
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn update_location(self, node: &Node) -> Self {
        Player {
            location: node.next_location().to_string(),
            ..self
        }
    }
    pub fn update_verb(self, verb: String) -> Self {
        Player {
            verb: verb.clone(),
            ..self
        }
    }

    pub fn get_location(&self) -> String {
        self.location.clone()
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
