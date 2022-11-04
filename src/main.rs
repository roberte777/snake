use snake::{genetic_ai::population::Population, gym::game::Game};

fn main() {
    let mut game = Game::new();
    let mut population = Population::new(2000, 5);
    population.evaluate(&mut game);
    let best = population.return_best();
    let board = best.final_board.as_ref().unwrap();
    println!("{} {}", board, best.fitness);
}
