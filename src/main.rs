use std::io::{self, Write};
mod game;
mod player;
mod user_input;

fn main() {
    println!("Starting the game !!!");
    let game_map = game::parse();
    let player1 = player::new(String::from("Rajib"));
    loop {
        println!(
            "> {}",
            game_map
                .locations
                .get(&player1.initial_location.to_string())
                .unwrap()
                .trim()
        );
        print!("~ ");
        io::stdout().flush().unwrap();

        if user_input::get().verb == "quit" {
            println!("Sorry to see you go!!");
            break;
        }
    }
}
