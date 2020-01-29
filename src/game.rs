use std::collections::HashMap;
use std::{fs::read_to_string, path::Path};

use player::Player;
//  ADVENTURES
//
//  CURRENT LIMITS:
//      9650 WORDS OF MESSAGE TEXT (LINES, LINSIZ).
//	750 TRAVEL OPTIONS (TRAVEL, TRVSIZ).
//	300 VOCABULARY WORDS (KTAB, ATAB, TABSIZ).
//	150 LOCATIONS (LTEXT, STEXT, KEY, COND, ABB, ATLOC, LOCSIZ).
//	100 OBJECTS (PLAC, PLACE, FIXD, FIXED, LINK (TWICE), PTEXT, PROP).
//	 35 "ACTION" VERBS (ACTSPK, VRBSIZ).
//	205 RANDOM MESSAGES (RTEXT, RTXSIZ).
//	 12 DIFFERENT PLAYER CLASSIFICATIONS (CTEXT, CVAL, CLSMAX).
//	 20 HINTS, LESS 3 (HINTLC, HINTED, HINTS, HNTSIZ).
//	 35 MAGIC MESSAGES (MTEXT, MAGSIZ).
//  THERE ARE ALSO LIMITS WHICH CANNOT BE EXCEEDED DUE TO THE STRUCTURE OF
//  THE DATABASE.  (E.G., THE VOCABULARY USES N/1000 TO DETERMINE WORD TYPE,
//  SO THERE CAN'T BE MORE THAN 1000 WORDS.)  THESE UPPER LIMITS ARE:
//	1000 NON-SYNONYMOUS VOCABULARY WORDS
//	300 LOCATIONS
//	100 OBJECTS

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    x: i32,
    y: i32,
    motions: Vec<i32>,
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

#[allow(dead_code)]
struct Element {}

pub struct GameMap {
    pub descriptions: HashMap<String, String>,
    pub vocabulary: HashMap<String, String>,
    pub maps: HashMap<String, Vec<Node>>,
}

impl GameMap {
    fn new(
        descriptions: HashMap<String, String>,
        maps: HashMap<String, Vec<Node>>,
        vocabulary: HashMap<String, String>,
    ) -> GameMap {
        GameMap {
            descriptions: descriptions,
            maps: maps,
            vocabulary: vocabulary,
        }
    }

    pub fn change_location(&self, gamer: &Player) -> Option<&Node> {
        let verb = &self.vocabulary.get(&gamer.verb).unwrap();
        let nodes = &self.maps.get(&gamer.location).unwrap();

        let node = nodes
            .iter()
            .find(|&n| n.motions.iter().any(|&m| m.to_string() == verb.trim()));
        node
    }
}

pub fn parse() -> GameMap {
    let game_data = load();
    let mut descriptions: HashMap<String, String> = HashMap::new();
    let mut maps: HashMap<String, Vec<Node>> = HashMap::new();
    let mut vocabulary: HashMap<String, String> = HashMap::new();

    let mut section = 1;
    for line in game_data.lines() {
        if line.starts_with("-1") {
            section = section + 1;
        }
        match section {
            1 => parse_location(line, &mut descriptions),
            3 => parse_travel_table(line, &mut maps),
            4 => parse_vocabulary(line, &mut vocabulary),
            _ => (),
        }
    }

    GameMap::new(descriptions, maps, vocabulary)
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

fn parse_travel_table(line: &str, maps: &mut HashMap<String, Vec<Node>>) {
    let line_iter: Vec<String> = line
        .split_whitespace()
        .map(|s| s.trim().to_string())
        .collect();

    if line_iter.len() == 1 {
        return;
    }

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

fn parse_location(line: &str, locations: &mut HashMap<String, String>) {
    let mut line_iter: Vec<String> = line
        .split_whitespace()
        .map(|s| s.trim_start().to_string())
        .collect();
    if line_iter.len() == 1 {
        return;
    }

    let position = line_iter.remove(0);
    let description = line_iter.join(" ");

    if locations.contains_key(&position) {
        let update_with = locations.get(&position).unwrap().to_owned();
        *locations.get_mut(&position).unwrap() = [update_with, description].join("\n")
    } else {
        locations.insert(position, description);
    }
}

fn parse_vocabulary(line: &str, vocabulary: &mut HashMap<String, String>) {
    let mut line_iter: Vec<String> = line
        .split_whitespace()
        .map(|s| s.trim_start().to_string())
        .collect();
    if line_iter.len() == 1 {
        return;
    }

    let value = line_iter.remove(0);
    let verb = line_iter.join(" ");
    vocabulary.insert(verb, value);
}
