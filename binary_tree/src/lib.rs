use std::fmt::Display;

pub struct Node<T> {
    pub value: T,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T: Display> Node<T> {
    pub fn print_inorder(&self) {
        if let Some(ref left) = self.left {
            left.print_inorder();
        }

        println!("{}", self.value);

        if let Some(ref right) = self.right {
            right.print_inorder();
        }
    }
}

impl<T: Ord> Node<T> {
    pub fn insert(&mut self, value: T) {
        if value < self.value {
            match self.left {
                Some(ref mut left_child) => left_child.insert(value),
                None => self.left = Some(Box::new(Node::new(value))),
            }
        } else {
            match self.right {
                Some(ref mut right_child) => right_child.insert(value),
                None => self.right = Some(Box::new(Node::new(value))),
            }
        }
    }
}
