use data_struct_lab::Node;

fn main() {
    let mut root = Node::new(42);
    
    root.left = Some(Box::new(Node::new(21)));
    root.right = Some(Box::new(Node::new(84)));
    
    root.insert(5);
    
    for i in [1,2,3,4,5] {
       root.insert(i); 
    }

    root.print_inorder();
    
    println!("{}", root.contains(5));
}
