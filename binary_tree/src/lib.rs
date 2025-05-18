use std::fmt::{self, Display, Formatter};

pub struct Node<T> {
    pub value: T,
    pub height: i32,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node { value, height: 1, left: None, right: None }
    }

    fn height(node: &Option<Box<Node<T>>>) -> i32 {
        node.as_ref().map_or(0, |n| n.height)
    }

    fn update_height(node: &mut Box<Node<T>>) {
        let hl = Self::height(&node.left);
        let hr = Self::height(&node.right);
        node.height = 1 + hl.max(hr);
    }

    fn balance_factor_node(node: &Box<Node<T>>) -> i32 {
        Self::height(&node.left) - Self::height(&node.right)
    }

    fn rotate_right(mut y: Box<Node<T>>) -> Box<Node<T>> {
        let mut x = y.left.take().expect("rotate_right called on node without left child");
        let t2 = x.right.take();

        x.right = Some(y);
        {
            let y_mut = x.right.as_mut().unwrap();
            y_mut.left = t2;
            Self::update_height(y_mut);
        }
        Self::update_height(&mut x);
        x
    }

    fn rotate_left(mut x: Box<Node<T>>) -> Box<Node<T>> {
        let mut y = x.right.take().expect("rotate_left called on node without right child");
        let t2 = y.left.take();

        y.left = Some(x);
        {
            let x_mut = y.left.as_mut().unwrap();
            x_mut.right = t2;
            Self::update_height(x_mut);
        }
        Self::update_height(&mut y);
        y
    }

    fn balance_node(mut node: Box<Node<T>>) -> Box<Node<T>>
    where T: Ord {
        Self::update_height(&mut node);
        let balance = Self::balance_factor_node(&node);

        if balance > 1 && Self::balance_factor_node(node.left.as_ref().unwrap()) >= 0 {
            return Self::rotate_right(node);
        }
        if balance > 1 && Self::balance_factor_node(node.left.as_ref().unwrap()) < 0 {
            let left = node.left.take().unwrap();
            node.left = Some(Self::rotate_left(left));
            return Self::rotate_right(node);
        }
        if balance < -1 && Self::balance_factor_node(node.right.as_ref().unwrap()) <= 0 {
            return Self::rotate_left(node);
        }
        if balance < -1 && Self::balance_factor_node(node.right.as_ref().unwrap()) > 0 {
            let right = node.right.take().unwrap();
            node.right = Some(Self::rotate_right(right));
            return Self::rotate_left(node);
        }
        node
    }
}

impl<T: Ord> Node<T> {
    fn insert_node(node: Option<Box<Node<T>>>, value: T) -> Box<Node<T>> {
        let current = if let Some(mut n) = node {
            if value < n.value {
                n.left = Some(Self::insert_node(n.left.take(), value));
            } else if value > n.value {
                n.right = Some(Self::insert_node(n.right.take(), value));
            }
            n
        } else {
            return Box::new(Node::new(value));
        };
        Self::balance_node(current)
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

pub struct Tree<T> {
    pub root: Option<Box<Node<T>>>,
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        Tree { root: None }
    }

    pub fn insert(&mut self, value: T)
    where T: Ord {
        let root = self.root.take();
        self.root = Some(Node::insert_node(root, value));
    }
}

impl<T: Display> Display for Tree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.root {
            Some(node) => write!(f, "{}", node),
            None => write!(f, "(empty tree)"),
        }
    }
}
