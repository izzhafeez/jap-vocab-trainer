pub struct Word {
  pub japanese: String,
  pub romaji: String,
  pub english: String,
  pub rank: i8
}

fn split_at_nth_char(s: &str, p: char, n: usize) -> Option<(&str, &str)> {
  s.match_indices(p).nth(n).map(|(index, _)| s.split_at(index))
}

pub fn get_word_from_line(line: &str) -> Word {
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
  Word {
    japanese: japanese,
    romaji: romaji,
    english: english,
    rank: 0
  }
}