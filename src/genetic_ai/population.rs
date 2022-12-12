use std::f32::INFINITY;

use crate::gym::{game::Game, snake::Direction};

use super::{Agent, Method};

pub struct Population {
    pub individuals: Vec<Agent>,
}
impl Population {
    pub fn new(size: u32, depth_limit: u32) -> Population {
        let mut individuals = Vec::new();

        for _ in 0..size / 2 {
            individuals.push(Agent::new(depth_limit, Method::Grow));
        }

        for _ in size / 2..size {
            individuals.push(Agent::new(depth_limit, Method::Full));
        }
        Population { individuals }
    }
    pub fn evaluate(&mut self, game: &mut Game) {
        println!("population size: {}", self.individuals.len());
        for individual in &mut self.individuals {
            let mut count = 0;

            while !game.lost {
                if count > 10000 {
                    println!("found infinite loop");
                    break;
                }

                let starting_score = game.score;
                let moves = game.get_possible_states();

                let best_move: Direction = moves
                    .iter()
                    .map(|(dir, game)| (dir, individual.evaluate(game)))
                    .max_by(|(_, a), (_, b)| {
                        if a.is_nan()
                            || b.is_nan()
                            || *a == INFINITY
                            || INFINITY == *b
                            || *a == -INFINITY
                            || -INFINITY == *b
                        {
                            panic!("found nan");
                        }
                        return a.partial_cmp(b).unwrap();
                    })
                    .unwrap()
                    .0
                    .to_owned();

                game.update_direction(best_move);
                game.update();
                game.display();

                if game.score == starting_score {
                    count += 1;
                } else {
                    count = 0;
                }
            }

            individual.final_board = Some(game.board.clone());
            individual.fitness = game.score;
            individual.final_snake = Some(game.snake.clone());
            game.reset();
        }
    }
    pub fn mutate(&mut self) {
        todo!()
    }
    pub fn crossover(&mut self) {
        todo!()
    }
    pub fn return_best(&self) -> &Agent {
        self.individuals
            .iter()
            .max_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap())
            .unwrap()
    }
}
