use std::io::{self, Write};
mod game;
mod player;
mod user_input;

fn main() {
    println!("Starting the game !!!");
    let game_map = game::parse();
    let mut gamer = player::Player::new("Rajib");
    loop {
        println!(
            "> {}",
            game_map
                .locations
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
        game_map.change_location(&mut gamer);
    }
}
