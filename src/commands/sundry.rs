use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("unable to read file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|s| s.expect("could not parse line"))
        .collect()
}

fn main() {
    // let data = fs::read_to_string("../../fortunes/fortunes").expect("unable to read file");
    // let data: Vec<String> = data.split("%").map(|s| s.to_string()).collect();
    // dbg!(&data);
    let lines = lines_from_file("../../fortunes/fortunes");
    for line in lines {
        println!("{:?}", line);
    }
}
