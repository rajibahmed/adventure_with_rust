extern crate colored;
mod game;
mod player;
mod user_input;

use colored::Colorize;
use game::Node;
use std::io::{self, Write};

fn main() {
    println!(
        "{}",
        r#"
     _______  ______            _______  _       _________          _______  _______ 
    (  ___  )(  __  \ |\     /|(  ____ \( (    /|\__   __/|\     /|(  ____ )(  ____ \ 
    | (   ) || (  \  )| )   ( || (    \/|  \  ( |   ) (   | )   ( || (    )|| (    \/
    | (___) || |   ) || |   | || (__    |   \ | |   | |   | |   | || (____)|| (__    
    |  ___  || |   | |( (   ) )|  __)   | (\ \) |   | |   | |   | ||     __)|  __)   
    | (   ) || |   ) | \ \_/ / | (      | | \   |   | |   | |   | || (\ (   | (      
    | )   ( || (__/  )  \   /  | (____/\| )  \  |   | |   | (___) || ) \ \__| (____/\
    |/     \|(______/    \_/   (_______/|/    )_)   )_(   (_______)|/   \__/(_______/
                                                                                     "#
        .red()
        .bold()
    );

    let game_map = game::parse();
    let mut gamer = player::Player::new("Rajib");
    loop {
        println!(
            "> {}",
            game_map
                .descriptions
                .get(&gamer.get_location())
                .unwrap()
                .trim()
                .green()
        );

        for (k, o) in game_map
            .objects
            .iter()
            .filter(|&(_, object)| object.location_contains(gamer.get_location()))
        {
            if let Some(x) = game_map.object_descriptions.get(&o.id) {
                println!("{}", x.description_message);
            }
        }

        print!("{}", "~ ");
        io::stdout().flush().unwrap();

        let input = user_input::get();

        if !game_map.valid_verb(&input.verb) {
            format!(
                "{} {}",
                String::from("Not a valid input").blue(),
                input.verb
            );
            continue;
        }

        gamer = gamer.update_input(&input);
        let change_to: Option<&Node> = game_map.change_location(&gamer);

        match change_to {
            Some(change_to) => gamer = gamer.update_location(change_to),
            None => (),
        }
    }
}
