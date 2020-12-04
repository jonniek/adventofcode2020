use std::io::{self, Read};

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  part1(&input);
  part2(&input);
}

fn is_tree(line: &str, pos: usize) -> bool {
  let x = pos % line.len();
  let c = line.chars().skip(x).next().unwrap();

  return match c {
    '#' => true,
    _ => false,
  };
}

fn get_slope_tree_count(dx: usize, dy: usize, input: &str) -> i64 {
  let lines: Vec<&str> = input.lines().collect();

  let mut count = 0;
  let mut x = 0;

  for line in lines.iter().step_by(dy) {
    if is_tree(line, x) {
      count += 1;
    }
    x += dx;
  }

  count
}


fn part1(input: &str) {
  let count = get_slope_tree_count(3, 1, input);
  println!("{:?}", count);
}


fn part2(input: &str) {
  let slopes: Vec<(usize, usize)> = vec!((1, 1), (3, 1), (5, 1), (7, 1), (1, 2));
  let product = slopes
    .into_iter()
    .map(|deltas| get_slope_tree_count(deltas.0, deltas.1, input))
    .fold(1 as i64, |total, next| total * next);

  println!("{:?}", product);
}
