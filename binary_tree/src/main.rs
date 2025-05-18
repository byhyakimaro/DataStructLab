use data_struct_lab::Tree;

fn main() {
    let mut tree = Tree::new();

    for &val in &[10, 20, 30, 40, 50, 25] {
        tree.insert(val);
    }

    println!("{}", tree);
}
