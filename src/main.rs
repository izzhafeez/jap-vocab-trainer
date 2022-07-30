use std::fs;

fn main() {
    let data: String = fs::read_to_string("src/data/Vocab - JLPT Vocab.csv").expect("Unable to read");
    let num_lines: usize = data.lines().count();
    let mut word_array: Vec<WordInfo> = Vec::<WordInfo>::with_capacity(num_lines);
    for line in data.lines() {
        let splitted_at_meaning: (&str, &str) = split_at_nth_char(&line, ',', 1).unwrap();

        let splitted_first: Vec<&str> = splitted_at_meaning.0.split(',').collect();
        let japanese: String = splitted_first[0].to_string();
        let romaji: String = splitted_first[1].to_string();

        let second: &str = splitted_at_meaning.1;
        let mut english: String = "".to_string();
        if second.contains('"') {
            let splitted_second: Vec<&str> = second.split("\"").collect();
            english = splitted_second[1].to_string();
        } else {
            let splitted_second: Vec<&str> = second.split(",").collect();
            english = splitted_second[1].to_string();
        }
        let mut word = WordInfo {
            japanese: japanese,
            romaji: romaji,
            english: english,
            rank: 0
        };
        word_array.push(word);
    }
}

fn split_at_nth_char(s: &str, p: char, n: usize) -> Option<(&str, &str)> {
    s.match_indices(p).nth(n).map(|(index, _)| s.split_at(index))
}

struct WordInfo {
    japanese: String,
    romaji: String,
    english: String,
    rank: i8
}