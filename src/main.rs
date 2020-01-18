use std::io::{self, Result, Write};
//promod game;
mod user_input;

fn main() -> Result<()> {
    println!("Starting the game !!!");
    //println!("{}", game::parse());
    print!("{}", "> ");
    io::stdout().flush().unwrap();
    println!("{}", user_input::get());
    Ok(())
}
