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
        self.y
    }

    #[allow(dead_code)]
    fn can_move(&self) -> bool {
        self.y <= 300
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

    pub fn change_location(&self, gamer: &Player) -> Option<&Node> {
        let verb = self.vocabulary.get(&gamer.verb).unwrap();
        let nodes = self.maps.get(&gamer.location).unwrap();

        let node = nodes
            .iter()
            .find(|&n| n.motions.iter().any(|&m| m.to_string() == verb.trim()));
        node
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

//  SECTION 3: TRAVEL TABLE.  EACH LINE CONTAINS A LOCATION NUMBER (X), A SECOND
//	LOCATION NUMBER (Y), AND A LIST OF MOTION NUMBERS (SEE SECTION 4).
//	EACH MOTION REPRESENTS A VERB WHICH WILL GO TO Y IF CURRENTLY AT X.
//	Y, IN TURN, IS INTERPRETED AS FOLLOWS.  LET M=Y/1000, N=Y MOD 1000.
//		IF N<=300	IT IS THE LOCATION TO GO TO.
//		IF 300<N<=500	N-300 IS USED IN A COMPUTED GOTO TO
//					A SECTION OF SPECIAL CODE.
//		IF N>500	MESSAGE N-500 FROM SECTION 6 IS PRINTED,
//					AND HE STAYS WHEREVER HE IS.
//	MEANWHILE, M SPECIFIES THE CONDITIONS ON THE MOTION.
//		IF M=0		IT'S UNCONDITIONAL.
//		IF 0<M<100	IT IS DONE WITH M% PROBABILITY.
//		IF M=100	UNCONDITIONAL, BUT FORBIDDEN TO DWARVES.
//		IF 100<M<=200	HE MUST BE CARRYING OBJECT M-100.
//		IF 200<M<=300	MUST BE CARRYING OR IN SAME ROOM AS M-200.
//		IF 300<M<=400	PROP(M MOD 100) MUST *NOT* BE 0.
//		IF 400<M<=500	PROP(M MOD 100) MUST *NOT* BE 1.
//		IF 500<M<=600	PROP(M MOD 100) MUST *NOT* BE 2, ETC.
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
