use std::io::{self, Result, Write};
mod game;
mod player;

fn main() -> Result<()> {
    println!("Starting the game !!!");
    let game_map = game::parse();

    let p = player::new(String::from("Rajib"));

    print!("{}", "> ");
    io::stdout().flush().unwrap();

    println!(
        "{}",
        game_map
            .locations
            .get(&p.initial_location.to_string())
            .unwrap()
    );
    Ok(())
}
