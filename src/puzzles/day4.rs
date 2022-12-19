use once_cell::sync::Lazy;
use regex::Regex;
use std::{fs, ops::Range};

static RANGE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+)-(\d+)").unwrap());

fn completely_overlaps<T: PartialOrd>(a: Range<T>, b: Range<T>) -> bool {
  (a.start <= b.start && a.end >= b.end) || (b.start <= a.start && b.end >= a.end)
}

fn partially_overlaps<T: PartialOrd>(a: Range<T>, b: Range<T>) -> bool {
  (a.start <= b.start && b.start <= a.end) || (b.start <= a.start && a.start <= b.end)
}

fn parse_range(range_str: &str) -> (u32, u32) {
  let caps = RANGE_REGEX.captures(range_str).unwrap();
  (caps[1].parse().unwrap(), caps[2].parse().unwrap())
}

pub fn day4(filename: &str) {
  let file = fs::read_to_string(filename).unwrap();
  let lines: Vec<_> = file.lines().collect();
  let (part_1, part_2) = lines
    .into_iter()
    .fold((0, 0), |(sum_complete, sum_partial), line| {
      let splits: Vec<_> = line.split(",").collect();
      let left = parse_range(splits[0]);
      let right = parse_range(splits[1]);
      let partial = partially_overlaps(left.0..left.1, right.0..right.1);
      let complete = completely_overlaps(left.0..left.1, right.0..right.1);

      (
        sum_complete + u32::from(complete),
        sum_partial + u32::from(partial),
      )
    });
  println!("Part 1: {part_1}, Part 2: {part_2}")
}
