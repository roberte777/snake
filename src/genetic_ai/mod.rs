use std::fmt::Display;

use rand::{distributions::Standard, prelude::*};
pub struct Agent {
    pub fitness: i32,
    pub gene: Tree,
}

impl Agent {
    pub fn new(depth_limit: u32, method: Method) -> Agent {
        Agent {
            fitness: 0,
            gene: Tree::new(depth_limit, method),
        }
    }
}

pub enum Method {
    Grow,
    Full,
}

pub struct Tree {
    root: Node,
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
        // use this to collapse the tree
        // let mut level = 0;
        // let mut stack = vec![];
        // let mut curr = &self.root;
        // loop {
        //     if curr.left.is_some() {
        //         stack.push(curr);
        //         curr = curr.left.as_ref().unwrap();
        //     } else {
        //         write!(f, "{}\n", curr)?;
        //         if curr.right.is_some() {
        //             curr = curr.right.as_ref().unwrap();
        //         } else {
        //             if stack.is_empty() {
        //                 break;
        //             }
        //             curr = stack.pop().unwrap();
        //             write!(f, "{}\n", curr)?;
        //             curr = curr.right.as_ref().unwrap();
        //         }
        //     }
        // }
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
#[derive(Debug)]
pub enum LeafNode {
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
pub enum NodeType {
    Internal(InternalNode),
    Leaf(LeafNode),
}
