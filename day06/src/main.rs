use std::io::{self, Read};
use std::collections::HashSet;

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  part1(&input);
  part2(&input);
}


fn part1(input: &str) {
  let groups: Vec<HashSet<char>> = input.split("\r\n\r\n").map(|group| {
    let chars: HashSet<char> = group.lines().flat_map(|line| line.chars()).collect();
    chars
  }).collect();

  let sum = groups.iter().fold(0, |sum, group| sum + group.len());

  println!("{:?}", sum)
}


fn part2(input: &str) {
  let groups: Vec<Vec<HashSet<char>>> = input.split("\r\n\r\n").map(|group| {
    group.lines().map(|line| line.chars().collect()).collect()
  }).collect();

  let group_intersections: Vec<Option<HashSet<char>>> = groups.iter().map(|group| {
    group.into_iter().fold(None, |result_option: Option<HashSet<char>>, set| match result_option {
      Some(result) => Some(result.into_iter().filter(|e| set.contains(e)).collect()),
      None => Some(set.clone()),
    })
  }).collect();

  let sum: usize = group_intersections.into_iter().map(|group_option| {
    return match group_option {
      Some(group) => group.len(),
      None => 0 as usize,
    };
  }).sum();

  println!("{:?}", sum)
}
