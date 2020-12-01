use std::io::{self, Read};

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  part1(&input);
  part2(&input);
}

const LIMIT: i32 = 2020;

fn part1(input: &str) {
  let numbers: Vec<i32> = input.lines().map(|v| v.parse::<i32>().unwrap()).collect();
  for (i, n) in numbers.iter().enumerate() {
    for m in numbers.iter().skip(i) {
      if n + m == LIMIT {
        println!("{}", n * m)
      }
    }
  }
}

fn part2(input: &str) {
  let numbers: Vec<i32> = input.lines().map(|v| v.parse::<i32>().unwrap()).collect();
  for (i, n) in numbers.iter().enumerate() {
    for (j, m) in numbers.iter().enumerate().skip(i) {
      if n + m >= LIMIT { continue }
      for b in numbers.iter().skip(j) {
        if n + m + b == LIMIT {
          println!("{}", n * m * b)
        }
      }
    }
  }
}
