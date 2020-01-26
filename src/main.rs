use std::io::{self, Write};
mod game;
mod player;
mod user_input;

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
                .get(&gamer.location.to_string())
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

        gamer.verb = input.verb.to_string();

        game_map.change_location(&gamer);
    }
}
