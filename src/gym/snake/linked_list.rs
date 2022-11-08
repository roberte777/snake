use std::fmt::{Display, Formatter};

use crate::gym::game::Point;

use super::Node;
pub struct LinkedList<T: Copy> {
    pub head: Option<Box<Node<T>>>,
    pub length: usize,
}

impl<T: Copy> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        let mut new_list = LinkedList::new(None);
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            new_list.push_back(node.value);
            current = node.next.as_ref();
        }
        new_list
    }
}

impl<T: Copy> LinkedList<T> {
    pub fn new(head: Option<Box<Node<T>>>) -> Self {
        let length = match Some(&head) {
            Some(_head) => 1,
            None => 0,
        };
        LinkedList { head, length }
    }
    pub fn front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }
    pub fn back(&self) -> Option<&T> {
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            if node.next.is_none() {
                return Some(&node.value);
            }
            current = node.next.as_ref();
        }
        None
    }

    pub fn push_front(&mut self, value: T) {
        let new_head = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_head);
        self.length += 1;
    }
    pub fn push_back(&mut self, value: T) {
        let mut tail = self.head.as_mut();
        let new_tail = Box::new(Node { value, next: None });
        while let Some(node) = tail {
            if node.next.is_none() {
                node.next = Some(new_tail);
                return;
            }
            tail = node.next.as_mut();
        }
        self.head = Some(new_tail);
        self.length += 1;
    }
    pub fn pop_back(&mut self) -> Option<T> {
        let mut current = &mut self.head;
        while let Some(node) = current {
            let possible_last = node.next.as_mut();
            if let Some(next) = possible_last {
                if next.next.is_none() {
                    let value = next.value;
                    node.next = None;
                    self.length -= 1;
                    return Some(value);
                }
            }

            current = &mut node.next;
        }
        None
    }
}

impl IntoIterator for LinkedList<Point> {
    type Item = Point;
    type IntoIter = LinkedListIterator<Point>;
    fn into_iter(self) -> Self::IntoIter {
        LinkedListIterator { current: self.head }
    }
}

pub struct LinkedListIterator<T: Copy> {
    current: Option<Box<Node<T>>>,
}

impl Iterator for LinkedListIterator<Point> {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node| {
            self.current = node.next;
            node.value
        })
    }
}

impl Display for LinkedList<Point> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            write!(f, "{}, ", node.value)?;
            current = node.next.as_ref();
        }
        Ok(())
    }
}
