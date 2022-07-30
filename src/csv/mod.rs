use std::fs;

pub fn read_csv(filename: &str) -> String {
  fs::read_to_string(filename).expect("Unable to read")
}