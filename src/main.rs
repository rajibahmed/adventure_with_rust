use std::io::{self, Write};
mod game;
mod player;
mod user_input;

fn main() {
    println!("Starting the game !!!");
    let game_map = game::parse();
    let mut player1 = player::Player::new("Rajib");
    loop {
        println!(
            "> {}",
            game_map
                .locations
                .get(&player1.location.to_string())
                .unwrap()
                .trim()
        );

        print!("~ ");
        io::stdout().flush().unwrap();
        let location = player1.location.parse::<u32>().unwrap();
        player1.location = (location + 1).to_string();

        if user_input::get().verb == "quit" {
            println!("Sorry to see you go!!");
            break;
        }
    }
}
