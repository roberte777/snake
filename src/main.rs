use snake::gym::{game::Game, snake::Direction};
fn main() {
    println!("Hello, world!");
    let mut game = Game::new();
    let mut count = 0;
    while !game.lost {
        if count == 5 {
            game.update_direction(Direction::Right);
        }
        game.display();
        game.update();
        if game.lost {
            println!("You lost!");
        }
        count += 1;
    }
}
