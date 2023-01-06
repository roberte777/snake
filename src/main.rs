use snake::{genetic_ai::population::Population, gym::game::Game};

fn main() {
    let mut game = Game::new();
    // let mut population = Population::new(1, 1);

    let mut population = Population::new(10, 1);
    population.evaluate(&mut game);
    let best = population.return_best();
    let board = best.final_board.as_ref().unwrap();
    let snake = best.final_snake.as_ref().unwrap();
    println!("{} {} {}", board, best.fitness, snake.body);
    println!("{}", best.gene);
}

#[cfg(test)]
mod tests {
    use super::*;
    use snake::genetic_ai::{Agent, InternalNode, LeafNode, Node, NodeType, Tree};

    #[test]
    fn it_works() {
        let mut new_node = Node::new(NodeType::Internal(InternalNode::Sub));
        new_node.left = Some(Box::new(Node::new(NodeType::Leaf(LeafNode::Random))));
        new_node.right = Some(Box::new(Node::new(NodeType::Leaf(LeafNode::AppleDistance))));
        let tree = Tree { root: new_node };

        let mut individuals = Vec::new();
        let custum_agent = Agent {
            fitness: 0,
            gene: tree,
            final_board: None,
            final_snake: None,
        };
        individuals.push(custum_agent);
        let mut population = Population { individuals };

        let mut game = Game::new();
        population.evaluate(&mut game);
        let best = population.return_best();
        let board = best.final_board.as_ref().unwrap();
        let snake = best.final_snake.as_ref().unwrap();
        println!("{} {} {}", board, best.fitness, snake.body);
        println!("{}", best.gene);
        assert!(true)
    }
}
