use std::fs;

fn main() {
    let data = fs::read_to_string("../../fortunes/fortunes").expect("unable to read file");
    let data: Vec<String> = data.split("%").map(|s| s.to_string()).collect();
    dbg!(&data);
}
