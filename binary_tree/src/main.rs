use data_struct_lab::Node;

fn main() {
    let mut root = Node::new(42);
    
    root.left = Some(Box::new(Node::new(21)));
    root.right = Some(Box::new(Node::new(84)));
    
    root.insert(5);

    root.print_inorder();
    
    println!("{}", root.contains(5));
}
