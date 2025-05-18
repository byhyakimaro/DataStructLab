use data_struct_lab::Tree;

fn main() {
    let mut tree = Tree::new();

    tree.insert(42);
    tree.insert(21);
    tree.insert(84);
    tree.insert(5);
    tree.insert(63);

    for elem in [1, 2, 3, 4] {
        tree.insert(elem);
    }

    println!("{}", tree);
}
