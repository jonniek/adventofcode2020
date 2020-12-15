use std::io::{self, Read};
use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  part1(&input);
  part2(&input);
}

#[test]
fn test_find_nth_number() {
  let input = "0,3,6";
  let numbers = parse_numbers(input);
  let result = find_nth_number(numbers, 2020);
  assert_eq!(result, 436);


  let input = "2,3,1";
  let numbers = parse_numbers(input);
  let result = find_nth_number(numbers, 2020);
  assert_eq!(result, 78);
}

fn find_nth_number(numbers: Vec<i32>, limit: i32) -> i32 {

  let mut spoken_map: HashMap<i32, VecDeque<i32>> = HashMap::new();

  let mut turn = 1;
  let mut spoken = 0;

  for n in numbers {
    let queue = spoken_map.entry(n).or_insert(VecDeque::new());
    queue.push_front(turn);
    spoken = n;
    turn += 1;
  }

  while turn <= limit {

    let queue = spoken_map.entry(spoken).or_insert(VecDeque::new());

    if queue.len() > 1 {
      // HAS BEEN SPOKEN BEFORE
      let mut iter = queue.iter();
      let next_spoken = iter.next().unwrap() - iter.next().unwrap();
      spoken = next_spoken;
    } else {
      // HAS NOT BEEN SPOKEN BEFORE
      spoken = 0;
    }

    let queue = spoken_map.entry(spoken).or_insert(VecDeque::new());
    queue.push_front(turn);

    turn += 1;
  }

  spoken
}

fn parse_numbers(input: &str) -> Vec<i32> {
  input.split(",").map(|n| n.parse::<i32>().unwrap()).collect()
}

fn part1(input: &str) {
  let numbers = parse_numbers(input);
  let result = find_nth_number(numbers, 2020);
  println!("{}", result);
}

fn part2(input: &str) {
  let numbers = parse_numbers(input);
  let result = find_nth_number(numbers, 30_000_000);
  println!("{}", result);
}