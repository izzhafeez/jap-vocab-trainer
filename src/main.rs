use std::fs;

fn main() {
    let data = fs::read_to_string("src/data/Vocab - JLPT Vocab.csv").expect("Unable to read");
    println!("{}", data);
}
