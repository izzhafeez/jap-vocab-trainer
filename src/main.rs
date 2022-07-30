use rand::seq::SliceRandom;
use std::io;

mod csv;
mod word;

fn main() {
  let data = csv::read_csv("src/data/Vocab - JLPT Vocab.csv");
  let num_lines: usize = data.lines().count();
  let mut word_array: Vec<word::Word> = Vec::<word::Word>::with_capacity(num_lines);

  for line in data.lines() {
    let mut word_info = word::get_word_from_line(line);
    word_array.push(word_info);
  }

  let choice: &word::Word = word_array.choose(&mut rand::thread_rng()).unwrap();
  println!("{}", choice.japanese);
}