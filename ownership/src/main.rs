fn main() {
    let first = String::from("Obi");
    let full = add_suffix(first);
    println!("{full}");
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" ");
    name.push_str("Baratt");
    name
}