use std::fmt::{self, Display, Formatter};

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

impl<T: Display> Display for Node<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(ref left) = self.left {
            write!(f, "({}) <- ", left)?;
        }
        write!(f, "{}", self.value)?;
        if let Some(ref right) = self.right {
            write!(f, " -> ({})", right)?;
        }
        Ok(())
    }
}

// Wrapper para a raiz da árvore, que pode estar vazia (None)
pub struct Tree<T> {
    pub root: Option<Box<Node<T>>>,
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        Tree { root: None }
    }

    pub fn insert(&mut self, value: T)
    where
        T: Ord,
    {
        Self::insert_node(&mut self.root, value);
    }

    fn insert_node(node: &mut Option<Box<Node<T>>>, value: T)
    where
        T: Ord,
    {
        match node {
            Some(n) => {
                if value < n.value {
                    Self::insert_node(&mut n.left, value);
                } else {
                    Self::insert_node(&mut n.right, value);
                }
            }
            None => {
                *node = Some(Box::new(Node::<T>::new(value)));
            }
        }
    }
}

impl<T: Display> Display for Tree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.root {
            Some(root_node) => write!(f, "{}", root_node),
            None => write!(f, "(empty tree)"),
        }
    }
}
