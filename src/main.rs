mod game;
mod player;

fn main() {
    println!("Starting the game !!!");
    game::load();
    game::parse();
    let player1 = player::build_user("rajib".to_string());
    player1.say_hello();
    player1.where_am_i();
}
