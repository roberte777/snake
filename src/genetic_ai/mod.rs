use rand::{distributions::Standard, prelude::*};
pub struct Agent {
    pub fitness: i32,
    pub gene: Tree,
}

impl Agent {
    pub fn new(depth_limit: u32) -> Agent {
        Agent {
            fitness: 0,
            gene: Tree::new(depth_limit),
        }
    }
}

struct Tree {
    root: Node,
}
impl Tree {
    fn new(depth_limit: u32) -> Tree {
        Tree {
            root: Node::new_internal(),
        }
    }
}
pub struct Node {
    node_type: NodeType,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}
impl Node {
    fn new(node_type: NodeType) -> Node {
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
enum InternalNode {
    Add,
    Sub,
    Mul,
    Div,
}
impl Distribution<InternalNode> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> InternalNode {
        match rng.gen_range(0..4) {
            0 => InternalNode::Add,
            1 => InternalNode::Sub,
            2 => InternalNode::Mul,
            3 => InternalNode::Div,
            _ => InternalNode::Add,
        }
    }
}
enum LeafNode {
    SnakeLength,
    SnakeDirection,
    AppleLocation,
}
impl Distribution<LeafNode> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> LeafNode {
        match rng.gen_range(0..3) {
            0 => LeafNode::SnakeLength,
            1 => LeafNode::SnakeDirection,
            2 => LeafNode::AppleLocation,
            _ => LeafNode::SnakeLength,
        }
    }
}
enum NodeType {
    Internal(InternalNode),
    Leaf(LeafNode),
}
