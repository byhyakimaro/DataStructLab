pub struct Node<T> {
    pub value: T,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
}

fn main() {
    let a: &str = "macaco";
    
    println!("{}", a);
    
    println!("{}", a);
}
