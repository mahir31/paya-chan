use std::fs;

fn main() {
    let data = fs::read_to_string("../../fortunes/fortunes").expect("unable to read file");
    println!("{}", data);
}
