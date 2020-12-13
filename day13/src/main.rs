use std::io::{self, Read};
use std::collections::BTreeSet;

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  part1(&input);
  part2(&input);
}

#[derive(Debug)]
struct Wait {
  buss_number: u64,
  wait_time: u64,
}

fn part1(input: &str) {

  let mut lines = input.lines();
  let time = lines.next().unwrap().parse::<u64>().unwrap();
  let numbers: BTreeSet<u64> = lines
    .next()
    .unwrap()
    .split(",")
    .filter(|n| *n != "x")
    .map(|n| n.parse::<u64>().unwrap())
    .collect();

  let mut wait: Option<Wait> = None;

  for n in numbers.iter() {
    let modulo = time % n;
    let wait_time = n - modulo;

    wait = match wait {
      Some(w) => {
        if wait_time < w.wait_time {
          Some(Wait { buss_number: *n, wait_time })
        } else {
          Some(w)
        }
      },
      None => Some(Wait { buss_number: *n, wait_time })
    }
  }

  let result = wait.unwrap();

  println!("{:?}", result.wait_time * result.buss_number);
}

fn part2(input: &str) {
}