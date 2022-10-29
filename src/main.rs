use snake::game::Game;
fn main() {
    println!("Hello, world!");
    let mut game = Game::new();
    while !game.lost {
        game.display();
        game.update();
        if game.lost {
            println!("You lost!");
        }
    }
}
