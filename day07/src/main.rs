use std::io::{self, Read};
use std::collections::HashSet;
use std::collections::HashMap;

extern crate regex;
use regex::Regex;


fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  part1(&input);
  part2(&input);
}

fn parse_children(input: &str) -> Vec<(String, i32)> {
  let children_regex = Regex::new(r"(\d+) (.+) bag").unwrap();
  match input == "no other bags." {
    true => Vec::new(),
    false => input.split(", ").map(|child| {
      let result = children_regex.captures(child).unwrap();
      let count: i32 = result.get(1).unwrap().as_str().parse().unwrap();
      let name: String = result.get(2).unwrap().as_str().to_string();
      (name, count)
    }).collect(),
  }
}

fn part1(input: &str) {
  let mut bag_map: HashMap<String, Vec<&str>> = HashMap::new();

  for description in input.lines() {
    let mut split = description.split(" bags contain ");

    let parent = split.next().unwrap();
    let children_string = split.next().unwrap();
    let children = parse_children(children_string);

    for bag in children {
      let parents = bag_map.entry(bag.0).or_insert(Vec::new());
      parents.push(parent);
    }
  }

  let shiny_gold_flat_children = flatten_bags("shiny gold", &bag_map);
  println!("{:?}", shiny_gold_flat_children.len() - 1);
}

// recursive set unions is kinda clumsy
fn flatten_bags(bag: &str, bag_map: &HashMap<String, Vec<&str>>) -> HashSet<String> {
  let mut current_set: HashSet<String> = [bag.to_string()].iter().cloned().collect();

  match bag_map.get(bag) {
    None => current_set,
    Some(bags) => {
      let parent_sets: Vec<HashSet<String>> = bags.iter().map(|bag| flatten_bags(bag, bag_map)).collect();
      for parent_set in parent_sets {
        current_set.extend(parent_set);
      }
      current_set
    }
  }
}

fn part2(input: &str) {
  let bag_map: HashMap<String, Vec<(String, i32)>> = input.lines().map(|line| {
    let mut split = line.split(" bags contain ");
    let parent = split.next().unwrap();
    let children_string = split.next().unwrap();
    let children = parse_children(children_string);

    (parent.to_string(), children)
  }).collect();

  let bag_count = calculate_nested_bag_count("shiny gold", 1, &bag_map);
  println!("{:?}", bag_count - 1);
}

fn calculate_nested_bag_count(bag_name: &str, multiplier: i32, bag_map: &HashMap<String, Vec<(String, i32)>>) -> i32 {
  multiplier + match bag_map.get(bag_name) {
    None => 0,
    Some(bags) =>
      bags
        .iter()
        .map(|bag| calculate_nested_bag_count(&bag.0, bag.1 * multiplier, bag_map))
        .sum()
  }
}
