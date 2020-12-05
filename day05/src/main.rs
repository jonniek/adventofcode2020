use std::io::{self, Read};
use std::collections::HashSet;

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  part1(&input);
  part2(&input);
}

fn binary_search(search: &str, upper_char: char) -> i32 {
  let mut upper_bound = (2 as i32).pow(search.len() as u32) - 1;
  let mut lower_bound = 0;

  for ch in search.chars() {
    let diff = upper_bound - lower_bound + 1;
    let middle = lower_bound + (diff / 2);

    if ch == upper_char {
      lower_bound = middle;
    } else {
      upper_bound = middle - 1;
    }
  }

  lower_bound
}

type Position = (i32, i32);

fn get_positions(boarding_pass: &str) -> Position {
  let (row_code, column_code) = boarding_pass.split_at(boarding_pass.len() - 3);
  let row = binary_search(row_code, 'B');
  let column = binary_search(column_code, 'R');

  (row, column)
}

fn get_position_id(pos: Position) -> i32 {
  pos.0 * 8 + pos.1
}

fn get_boarding_pass_id(boarding_pass: &str) -> i32 {
  let (row, column) = get_positions(boarding_pass);
  get_position_id((row, column))
}


fn part1(input: &str) {
  let boarding_passes: Vec<&str> = input.lines().collect();
  let largest_id = boarding_passes.iter().map(|boarding_pass| get_boarding_pass_id(boarding_pass)).max();
  println!("{:?}", largest_id.unwrap());
}


fn part2(input: &str) {
  let boarding_passes: Vec<&str> = input.lines().collect();

  let all_positions: HashSet<Position> = boarding_passes.iter().map(|boarding_pass| get_positions(boarding_pass)).collect();
  let all_ids: HashSet<i32> = boarding_passes.iter().map(|boarding_pass| get_boarding_pass_id(boarding_pass)).collect();

  // skip first and last row
  for row in 1..127 {

    // check all columns
    for column in 0..=7 {

      let pos = (row, column);
      if !all_positions.contains(&pos) {

        let id = get_position_id(pos);
        if all_ids.contains(&(id - 1)) && all_ids.contains(&(id + 1)) {
          println!("{}", id);
        }

      }

    }
  }
}
