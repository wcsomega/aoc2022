use std::fs;

const CHAR_LOOKUP: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn get_char_priority(char: &str) -> Option<u32> {
  match CHAR_LOOKUP.find(char) {
    Some(result) => Some(result as u32 + 1),
    None => None,
  }
}

fn get_common_char<'a>(lines: &[&'a str]) -> Option<&'a str> {
  for (i, ch) in lines[0].char_indices() {
    if lines[1..].iter().cloned().all(|line| line.contains(ch)) {
      return Some(&lines[0][i..i + 1]);
    }
  }
  None
}

pub fn day3(filename: &str) {
  let file = fs::read_to_string(filename).unwrap();
  let lines = file.lines().collect::<Vec<_>>();
  let part1 = lines.iter().cloned().fold(0, |sum, line| {
    let (left, right) = line.split_at(line.len() / 2);
    sum
      + get_common_char(&[left, right])
        .and_then(get_char_priority)
        .unwrap_or(0)
  });
  let part2 = lines.chunks(3).fold(0, |sum, group| {
    sum
      + get_common_char(group)
        .and_then(get_char_priority)
        .unwrap_or(0)
  });
  println!("Part 1: {}, Part 2: {}", part1, part2)
}
