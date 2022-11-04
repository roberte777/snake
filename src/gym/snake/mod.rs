use crate::gym::game::Point;

use self::linked_list::LinkedList;
pub mod linked_list;

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
pub struct Snake {
    pub body: LinkedList<Point>,
    pub direction: Direction,
}

impl Snake {
    pub fn new() -> Self {
        Snake {
            body: LinkedList::new(Some(Box::new(Node::new(Point { x: 0, y: 0 })))),
            direction: Direction::Down,
        }
    }
    pub fn slither(&mut self) {
        let mut head = self.body.front().unwrap().clone();
        match self.direction {
            Direction::Up => head.y -= 1,
            Direction::Down => head.y += 1,
            Direction::Left => head.x -= 1,
            Direction::Right => head.x += 1,
        }
        self.body.push_front(head);
        self.body.pop_back();
    }
}

#[derive(Clone)]
pub struct Node<T: Copy> {
    pub value: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Copy> Node<T> {
    fn new(value: T) -> Self {
        Node { value, next: None }
    }
}
