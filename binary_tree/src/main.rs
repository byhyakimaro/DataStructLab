use data_struct_lab::Tree;

fn main() {
    let mut tree = Tree::new();

    for v in [30, 20, 40, 10, 25, 50] {
        tree.insert(v);
    }
    
    println!("Contém 25? {}", tree.contains(&25));

    println!("{}", tree.to_json());
}
