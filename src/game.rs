use std::collections::HashMap;
use std::{fs::read_to_string, path::Path};

use player::Player;

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    x: i32,
    y: i32,
    motions: Vec<i32>,
}

#[derive(Debug)]
pub struct Element {
    object: String,
    locations: Vec<String>,
}

impl Element {
    fn new(object: String, locations: Vec<String>) -> Self {
        Element {
            object: object,
            locations: locations,
        }
    }
}

impl Node {
    fn new(x: i32, y: i32, motions: Vec<i32>) -> Node {
        Node {
            x: x,
            y: y,
            motions: motions,
        }
    }

    pub fn next_location(&self) -> i32 {
        let n = self.get_n();
        let m = self.get_m();

        match (n, m) {
            (.., 101..=200) => {
                //check for stuff object m-100
                println!("check object m-100");
                self.x
            }
            (.., 201..=300) => {
                // MUST BE CARRYING OR IN SAME ROOM AS M-200
                println!("check object m-200");
                self.x
            }
            // base condition
            (0..=300, m) if m == 0 || m == 100 => {
                println!("base condiftion");
                self.y
            }
            // return with probability
            (0..=300, 1..=99) => {
                println!("base condiftion");
                self.y
            }
            //N-300 IS USED IN A COMPUTED GOTO TO
            //A SECTION OF SPECIAL CODE.
            (300..=500, _) => self.x,
            // MESSAGE N-500 FROM SECTION 6 IS PRINTED,
            // AND HE STAYS WHEREVER HE IS.
            (500..=10000, _) => self.x,
            _ => self.x,
        }
    }

    fn get_n(&self) -> i32 {
        self.y % 1000
    }

    fn get_m(&self) -> i32 {
        self.y / 1000
    }
}

pub struct GameMap {
    pub descriptions: HashMap<String, String>,
    pub vocabulary: HashMap<String, String>,
    pub maps: HashMap<String, Vec<Node>>,
    pub arbitary: HashMap<String, String>,
    pub objects: HashMap<String, Element>,
    pub action_verbs: HashMap<String, String>,
}

impl GameMap {
    fn new(
        descriptions: HashMap<String, String>,
        maps: HashMap<String, Vec<Node>>,
        vocabulary: HashMap<String, String>,
        arbitary: HashMap<String, String>,
        objects: HashMap<String, Element>,
        action_verbs: HashMap<String, String>,
    ) -> GameMap {
        GameMap {
            descriptions: descriptions,
            maps: maps,
            vocabulary: vocabulary,
            arbitary: arbitary,
            objects: objects,
            action_verbs: action_verbs,
        }
    }

    //  SECTION 4: VOCABULARY.  EACH LINE CONTAINS A NUMBER (N), A TAB, AND A
    //	FIVE-LETTER WORD.  CALL M=N/1000.  IF M=0, THEN THE WORD IS A MOTION
    //	VERB FOR USE IN TRAVELLING (SEE SECTION 3).  ELSE, IF M=1, THE WORD IS
    //	AN OBJECT.  ELSE, IF M=2, THE WORD IS AN ACTION VERB (SUCH AS "CARRY"
    //	OR "ATTACK").  ELSE, IF M=3, THE WORD IS A SPECIAL CASE VERB (SUCH AS
    //	"DIG") AND N MOD 1000 IS AN INDEX INTO SECTION 6.  OBJECTS FROM 50 TO
    //	(CURRENTLY, ANYWAY) 79 ARE CONSIDERED TREASURES (FOR PIRATE, CLOSEOUT).
    pub fn change_location(&self, gamer: &Player) -> Option<&Node> {
        let verb = self.vocabulary.get(&gamer.verb).unwrap();
        // let noun = &gamer.noun;
        let nodes = self.maps.get(&gamer.location).unwrap();

        let n = verb.parse::<i32>().unwrap();
        let m = n % 1000;

        match n / 1000 {
            0 => {
                println!("is motion {}", verb);
                let node = nodes
                    .iter()
                    .find(|&n| n.motions.iter().any(|&m| m.to_string() == verb.trim()));
                node
            }
            1 => {
                println!("is object {}", verb);
                None
            }
            2 => {
                println!("is action {}", verb);
                None
            }
            3 => {
                //&gamer.add_message()
                None
            }
            _ => {
                println!("{}.. {}", n, verb);
                None
            }
        }
    }

    pub fn valid_verb(&self, verb: &String) -> bool {
        self.vocabulary.contains_key(verb)
    }
}

pub fn parse() -> GameMap {
    let game_data = load();
    let mut descriptions: HashMap<String, String> = HashMap::new();
    let mut maps: HashMap<String, Vec<Node>> = HashMap::new();
    let mut vocabulary: HashMap<String, String> = HashMap::new();
    let mut arbitary: HashMap<String, String> = HashMap::new();
    let mut objects: HashMap<String, Element> = HashMap::new();
    let mut action_verbs: HashMap<String, String> = HashMap::new();

    let mut section = 1;
    for line in game_data.lines() {
        if line.starts_with("-1") {
            section = section + 1;
        }

        let line_iter = split_line(line);
        if line_iter.len() == 1 {
            continue;
        }
        match section {
            1 => parse_location(line_iter, &mut descriptions),
            3 => parse_travel_table(line_iter, &mut maps),
            4 => parse_vocabulary(line_iter, &mut vocabulary),
            6 => parse_location(line_iter, &mut arbitary),
            7 => parse_objects(line_iter, &mut objects),
            8 => parse_action_verb(line_iter, &mut action_verbs),
            _ => (),
        }
    }

    GameMap::new(
        descriptions,
        maps,
        vocabulary,
        arbitary,
        objects,
        action_verbs,
    )
}

fn split_line(line: &str) -> Vec<String> {
    line.split_whitespace()
        .map(|s| s.trim().to_string())
        .collect()
}

fn load() -> String {
    read_to_string(Path::new("./src/advent.dat")).expect("Somethis whent wrong")
}

//	IF THE CONDITION (IF ANY) IS NOT MET, THEN THE NEXT *DIFFERENT*
//	"DESTINATION" VALUE IS USED (UNLESS IT FAILS TO MEET *ITS* CONDITIONS,
//	IN WHICH CASE THE NEXT IS FOUND, ETC.).  TYPICALLY, THE NEXT DEST WILL
//	BE FOR ONE OF THE SAME VERBS, SO THAT ITS ONLY USE IS AS THE ALTERNATE
//	DESTINATION FOR THOSE VERBS.  FOR INSTANCE:
//		15	110022	29	31	34	35	23	43
//		15	14	29
//	THIS SAYS THAT, FROM LOC 15, ANY OF THE VERBS 29, 31, ETC., WILL TAKE
//	HIM TO 22 IF HE'S CARRYING OBJECT 10, AND OTHERWISE WILL GO TO 14.
//		11	303008	49
//		11	9	50
//	THIS SAYS THAT, FROM 11, 49 TAKES HIM TO 8 UNLESS PROP(3)=0, IN WHICH
//	CASE HE GOES TO 9.  VERB 50 TAKES HIM TO 9 REGARDLESS OF PROP(3).

fn parse_travel_table(line_iter: Vec<String>, maps: &mut HashMap<String, Vec<Node>>) {
    let x = &line_iter[0].to_string().parse::<i32>().unwrap().clone();
    let y = &line_iter[1].to_string().parse::<i32>().unwrap().clone();

    let mut motions: Vec<i32> = vec![];

    let mut nodes: Vec<Node> = Vec::new();

    for i in line_iter.iter().skip(2) {
        motions.push(i.to_string().parse::<i32>().unwrap());
    }

    let check_location = x.to_string();
    if maps.contains_key(&check_location) {
        nodes = maps.get(&check_location).unwrap().to_vec();
    }
    nodes.push(Node::new(*x, *y, motions));
    maps.insert(x.to_string(), nodes);
}

fn parse_objects(line_iter: Vec<String>, objects: &mut HashMap<String, Element>) {
    let object = &line_iter[0].to_string();
    let locations: Vec<String> = line_iter.iter().skip(1).map(ToString::to_string).collect();
    if locations.len() == 0 {
        let locations = vec!["0".to_string()];
        objects.insert(
            object.to_string(),
            Element::new(object.to_string(), locations),
        );
    }
    objects.insert(
        object.to_string(),
        Element::new(object.to_string(), locations),
    );
}

fn parse_location(line_iter: Vec<String>, locations: &mut HashMap<String, String>) {
    let position = line_iter[0].to_string();
    let description = line_iter[1..].join(" ");

    if locations.contains_key(&position) {
        let update_with = locations.get(&position).unwrap().to_owned();
        *locations.get_mut(&position).unwrap() = [update_with, description].join("\n")
    } else {
        locations.insert(position, description);
    }
}

fn parse_vocabulary(line_iter: Vec<String>, vocabulary: &mut HashMap<String, String>) {
    let value = line_iter[0].to_string();
    let verb = line_iter[1..].join(" ");
    vocabulary.insert(verb, value);
}

fn parse_action_verb(line_iter: Vec<String>, action_verbs: &mut HashMap<String, String>) {
    let action = line_iter[0].to_string();
    let verb = line_iter[1].to_string();
    action_verbs.insert(action, verb);
}
