use data_struct_lab::Tree;

fn main() {
    let mut tree = Tree::new();
 
    tree.insert(42);
    tree.insert(21);
    tree.insert(84);
    tree.insert(5);
    tree.insert(63);

    println!("{}", tree);
}
