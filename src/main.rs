use rand::prelude::*;
use snake::gym::{game::Game, snake::Direction};
fn main() {
    let mut r = rand::thread_rng();
    let mut game = Game::new();
    let mut count = 0;
    while !game.lost {
        if count % 5 == 0 {
            let idx = r.gen_range(0..4);
            match idx {
                0 => game.update_direction(Direction::Up),
                1 => game.update_direction(Direction::Down),
                2 => game.update_direction(Direction::Left),
                3 => game.update_direction(Direction::Right),
                _ => {}
            }
        }
        game.display();
        game.update();
        count += 1;
    }
    println!("Game Over: {}", game.score);
}
