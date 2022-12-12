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

    pub fn check_self_collision(&self) -> bool {
        let head = self.body.front().unwrap();
        let mut current = self.body.head.as_ref().unwrap().next.as_ref();
        while let Some(node) = current {
            if node.value.x == (*head).x && node.value.y == (*head).y {
                return true;
            }
            current = node.next.as_ref();
        }
        false
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
