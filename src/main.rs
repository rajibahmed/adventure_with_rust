mod game;
use std::io::{self, Write};
mod player;
mod user_input;

use game::Node;

fn main() {
    println!(
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
        );

        print!("~ ");
        io::stdout().flush().unwrap();

        let input = user_input::get();
        if input.verb == "quit" {
            println!("Sorry to see you go!!");
            break;
        }
        if !game_map.valid_verb(&input.verb) {
            println!("Not a valid input: {}", input.verb);
            continue;
        }

        gamer = gamer.update_verb(input.verb.to_string());
        let change_to: Option<&Node> = game_map.change_location(&gamer);

        match change_to {
            Some(change_to) => gamer = gamer.update_location(change_to),
            None => (),
        }
    }
}
