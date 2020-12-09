use std::io::{self, Read};
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  part1(&input);
  part2(&input);
}

fn has_two_sum(target: i64, list: &VecDeque<i64>) -> bool {
  let mut set: HashSet<i64> = HashSet::new();

  for number in list {
    let diff = target - *number;
    if set.contains(&diff) {
      return true
    }
    set.insert(*number);
  }

  false
}

fn compute_invalid_number(input: &str, preamble_size: usize) -> Option<i64> {
  let full_input: Vec<i64> = input.lines().map(|line| line.parse().unwrap()).collect();
  let mut preamble: VecDeque<i64> = full_input.iter().take(preamble_size).cloned().collect();

  for next in full_input.iter().skip(preamble_size) {
    if !has_two_sum(*next, &preamble) {
      return Some(*next);
    }

    preamble.pop_front();
    preamble.push_back(*next);
  }

  None
}

fn part1(input: &str) {
  let invalid_number = compute_invalid_number(input, 25);
  println!("{}", invalid_number.unwrap());
}

fn part2(input: &str) {
  let all_numbers: Vec<i64> = input.lines().map(|line| line.parse().unwrap()).collect();
  let invalid_number = compute_invalid_number(input, 25).unwrap();

  let mut num_queue: VecDeque<i64> = VecDeque::new();
  for number in all_numbers.iter() {
    num_queue.push_back(*number);

    if num_queue.iter().sum::<i64>() == invalid_number {
      let min = num_queue.iter().min().unwrap();
      let max = num_queue.iter().max().unwrap();
      println!("{}", min + max);
      break;
    }

    while num_queue.iter().sum::<i64>() > invalid_number {
      num_queue.pop_front();
    }
  }
}
