mod game;
mod player;

fn main() {
    println!("Starting the game !!!");

    game::load();
    game::parse();

    let player1 = player::Player{ name: "rajib", location: 2 };
    player1.say_hello();
}
