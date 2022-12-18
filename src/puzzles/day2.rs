use std::fs;

fn strategy_1(line: &str) -> i32 {
  //each line is of the form "A X"
  //one character for the opponent
  //and one character for us
  //separated by a single whitespace
  let (opp, me) = line.split_once(" ").unwrap();
  match (opp, me) {
    ("A", "X") => 4, //1+3: rock     rock     draw
    ("A", "Y") => 8, //2+6: rock     paper    win
    ("A", "Z") => 3, //3+0: rock     scissors loss
    ("B", "X") => 1, //1+0: paper    rock     loss
    ("B", "Y") => 5, //2+3: paper    paper    draw
    ("B", "Z") => 9, //3+6: paper    scissors win
    ("C", "X") => 7, //1+6: scissors rock     win
    ("C", "Y") => 2, //2+0: scissors paper    loss
    ("C", "Z") => 6, //3+3: scissors scissors draw
    _ => unreachable!(),
  }
}
//rock: 1, paper: 2, scissors: 3
//win: 6, draw: 3, lose: 0
fn strategy_2(line: &str) -> i32 {
  let (opp, me) = line.split_once(" ").unwrap();
  match (opp, me) {
    ("A", "X") => 3, //0+3: rock,     lose, scissors
    ("A", "Y") => 4, //3+1: rock,     draw, rock
    ("A", "Z") => 8, //6+2: rock,     win,  paper
    ("B", "X") => 1, //0+1: paper,    lose, rock
    ("B", "Y") => 5, //3+2: paper,    draw, paper
    ("B", "Z") => 9, //6+3: paper,    win,  scissors
    ("C", "X") => 2, //0+2: scissors, lose, paper
    ("C", "Y") => 6, //3+3: scissors, draw, scissors
    ("C", "Z") => 7, //6+1: scissors, win,  rock
    _ => unreachable!(),
  }
}

pub fn day2(filename: &str) {
  let file = fs::read_to_string(filename).unwrap();
  let results = file.lines().into_iter().fold((0, 0), |prev, cur| {
    (prev.0 + strategy_1(cur), prev.1 + strategy_2(cur))
  });
  println!("Part 1: {:?}, Part 2: {:?}", results.0, results.1);
}
