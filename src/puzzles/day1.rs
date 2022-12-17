use std::fs;

pub fn day1(filename: &str) {
  let file = fs::read_to_string(filename).expect("couldn't read file");
  let max_3_totals = file
    .split("\n\n") //split on double newline
    .map(|elf| {
      elf
        .lines() //split into lines
        .map(|food| food.parse::<i32>().unwrap()) //parse into numbers
        .fold(0, |prev, cur| prev + cur) //add those numbers together
    })
    //take biggest 3
    .fold((0, 0, 0), |prev, curr| {
      if curr > prev.0 {
        (curr, prev.0, prev.1)
      } else if curr > prev.1 {
        (prev.0, curr, prev.1)
      } else if curr > prev.2 {
        (prev.0, prev.1, curr)
      } else {
        prev
      }
    });
  println!(
    "Part 1: {:?}, Part 2: {:?}",
    max_3_totals.0,
    max_3_totals.0 + max_3_totals.1 + max_3_totals.2
  );
}
