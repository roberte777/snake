pub mod population;
use std::fmt::Display;

use rand::{distributions::Standard, prelude::*};

use crate::gym::{
    game::{Board, Game},
    snake::{Direction, Snake},
};
pub struct Agent {
    pub fitness: u32,
    pub gene: Tree,
    pub final_board: Option<Board>,
    pub final_snake: Option<Snake>,
}

impl Agent {
    pub fn new(depth_limit: u32, method: Method) -> Agent {
        Agent {
            fitness: 0,
            gene: Tree::new(depth_limit, method),
            final_board: None,
            final_snake: None,
        }
    }
    pub fn evaluate(&mut self, state: &Game) -> f32 {
        //decide which direction to go
        // let current = &self.gene.root;
        // self.fitness = self.eval_recurse(state, current);
        let current = &self.gene.root;
        let test = self.eval_recurse(state, current);
        return test;
    }
    pub fn eval_recurse(&self, state: &Game, node: &Node) -> f32 {
        match &node.node_type {
            NodeType::Leaf(leaf) => {
                let temp = leaf.get_value(state);
                if temp.is_nan() {
                    println!("found nan in leaf");
                }
                return leaf.get_value(state);
            }
            NodeType::Internal(branch) => {
                let left = self.eval_recurse(state, &node.left.as_ref().unwrap());
                let right = self.eval_recurse(state, &node.right.as_ref().unwrap());
                if left.is_nan() || right.is_nan() {
                    println!("found nan in internal");
                }
                return branch.eval(left, right);
            }
        }
    }
}

pub enum Method {
    Grow,
    Full,
}

pub struct Tree {
    pub root: Node,
}
impl Tree {
    fn new(depth_limit: u32, method: Method) -> Tree {
        let starting_node = match method {
            Method::Grow => Tree::grow(depth_limit, 0, 0.5),
            Method::Full => Tree::full(depth_limit, 0),
        };
        Tree {
            root: starting_node,
        }
    }

    fn grow(depth_limit: u32, curr_level: u32, threshhold: f32) -> Node {
        if curr_level == depth_limit {
            return Node::new_leaf();
        }
        let mut rng = rand::thread_rng();
        let rand_num: f32 = rng.gen();
        if rand_num < threshhold {
            let mut new_node = Node::new_internal();
            new_node.left = Some(Box::new(Tree::grow(
                depth_limit,
                curr_level + 1,
                threshhold,
            )));
            new_node.right = Some(Box::new(Tree::grow(
                depth_limit,
                curr_level + 1,
                threshhold,
            )));
            return new_node;
        } else {
            return Node::new_leaf();
        }
    }
    fn full(depth_limit: u32, curr_level: u32) -> Node {
        if curr_level == 0 {
            let mut new_node = Node::new_internal();
            new_node.left = Some(Box::new(Tree::full(depth_limit, curr_level + 1)));
            new_node.right = Some(Box::new(Tree::full(depth_limit, curr_level + 1)));
            return new_node;
        } else if curr_level == depth_limit {
            return Node::new_leaf();
        } else {
            let mut new_node = Node::new_internal();
            new_node.left = Some(Box::new(Tree::full(depth_limit, curr_level + 1)));
            new_node.right = Some(Box::new(Tree::full(depth_limit, curr_level + 1)));
            return new_node;
        }
    }
    fn print(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.print_recurse(&self.root, 0, f)
    }
    fn print_recurse(
        &self,
        node: &Node,
        depth: u32,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        for _ in 0..depth {
            write!(f, "|")?;
        }
        write!(f, "{}\n", node)?;
        if let Some(left) = &node.left {
            self.print_recurse(&left, depth + 1, f)?;
        }
        if let Some(right) = &node.right {
            self.print_recurse(&right, depth + 1, f)?;
        }
        Ok(())
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.print(f)?;
        Ok(())
    }
}

pub struct Node {
    pub node_type: NodeType,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}
impl Node {
    pub fn new(node_type: NodeType) -> Node {
        Node {
            node_type,
            left: None,
            right: None,
        }
    }
    fn new_leaf() -> Node {
        let node: LeafNode = rand::random();
        Node {
            node_type: NodeType::Leaf(node),
            left: None,
            right: None,
        }
    }
    fn new_internal() -> Node {
        let node: InternalNode = rand::random();
        Node {
            node_type: NodeType::Internal(node),
            left: None,
            right: None,
        }
    }
}
impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.node_type {
            NodeType::Leaf(leaf) => write!(f, "{:?}", leaf),
            NodeType::Internal(internal) => write!(f, "{:?}", internal),
        }
    }
}

#[derive(Debug)]
pub enum InternalNode {
    Add,
    Sub,
    Mul,
    Div,
    Max,
    Min,
}
impl InternalNode {
    fn eval(&self, left: f32, right: f32) -> f32 {
        match self {
            InternalNode::Add => left + right,
            InternalNode::Sub => left - right,
            InternalNode::Mul => left * right,
            InternalNode::Div => {
                if right == 0.0 {
                    return left;
                }
                left / right
            }
            InternalNode::Max => left.max(right),
            InternalNode::Min => left.min(right),
        }
    }
}
impl Distribution<InternalNode> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> InternalNode {
        match rng.gen_range(0..6) {
            0 => InternalNode::Add,
            1 => InternalNode::Sub,
            2 => InternalNode::Mul,
            3 => InternalNode::Div,
            4 => InternalNode::Max,
            5 => InternalNode::Min,
            _ => InternalNode::Add,
        }
    }
}
#[derive(Debug)]
pub enum LeafNode {
    SnakeLength,
    SnakeDirection,
    AppleDistance,
    AppleDirection,
    Random,
    Row,
    Column,
}
impl LeafNode {
    fn get_value(&self, game: &Game) -> f32 {
        match self {
            LeafNode::SnakeLength => game.snake.body.length as f32,
            LeafNode::SnakeDirection => match game.snake.direction {
                Direction::Up => 0.0,
                Direction::Down => 1.0,
                Direction::Left => 2.0,
                Direction::Right => 3.0,
            },
            LeafNode::AppleDistance => {
                let apple_loc = game.apple.location;
                let snake_head = match &game.snake.body.head {
                    Some(head) => head,
                    None => return 0.0,
                };
                //manhattan distance
                ((apple_loc.x - snake_head.value.x).abs()
                    + (apple_loc.y - snake_head.value.y).abs()) as f32
            }
            LeafNode::AppleDirection => {
                let apple_loc = game.apple.location;
                let snake_head = match &game.snake.body.head {
                    Some(head) => head,
                    None => return 0.0,
                };
                let x_diff = apple_loc.x as f32 - snake_head.value.x as f32;
                let y_diff = apple_loc.y as f32 - snake_head.value.y as f32;
                let angle = y_diff.atan2(x_diff);
                let snake_angle = match game.snake.direction {
                    Direction::Up => 0.0,
                    Direction::Down => 1.0,
                    Direction::Left => 2.0,
                    Direction::Right => 3.0,
                };
                let angle_diff = angle - snake_angle;
                angle_diff
            }
            LeafNode::Random => rand::random(),
            LeafNode::Row => game.snake.body.head.as_ref().unwrap().value.y as f32,
            LeafNode::Column => game.snake.body.head.as_ref().unwrap().value.x as f32,
        }
    }
}
impl Distribution<LeafNode> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> LeafNode {
        match rng.gen_range(0..7) {
            0 => LeafNode::SnakeLength,
            1 => LeafNode::SnakeDirection,
            2 => LeafNode::AppleDistance,
            3 => LeafNode::AppleDirection,
            4 => LeafNode::Random,
            5 => LeafNode::Row,
            6 => LeafNode::Column,
            _ => LeafNode::SnakeLength,
        }
    }
}
pub enum NodeType {
    Internal(InternalNode),
    Leaf(LeafNode),
}
