use std::io::{self, Read};
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  part1(&input);
  part2(&input);
}


#[derive(Debug)]
struct Notes {
  fields: HashMap<String, HashSet<i32>>,
  ticket: Vec<i32>,
  nearby_tickets: Vec<Vec<i32>>,
}

fn parse_input(input: &str) -> Notes {
  let mut sections = input.split("\r\n\r\n");

  let fields = sections.next().unwrap().lines().map(|line| {
    let mut split = line.split(": ");
    let name = split.next().unwrap().to_string();

    let range_values: HashSet<i32> = split
      .next()
      .unwrap()
      .replace(" or", "")
      .split(" ")
      .flat_map(|range| {
        let mut split = range.split("-");
        let lower = split.next().unwrap().parse::<i32>().unwrap();
        let upper = split.next().unwrap().parse::<i32>().unwrap();
        (lower..=upper).collect::<Vec<i32>>()
      }).collect();

    (name, range_values)
  }).collect();

  let ticket = sections
    .next()
    .unwrap()
    .lines()
    .skip(1)
    .next()
    .unwrap()
    .split(",")
    .map(|n| n.parse::<i32>().unwrap())
    .collect();

  let nearby_tickets = sections
    .next()
    .unwrap()
    .lines()
    .skip(1)
    .map(|line|
      line
      .split(",")
      .map(|n| n.parse::<i32>().unwrap()).collect()
    )
    .collect();

  Notes {
    fields,
    ticket,
    nearby_tickets,
  }

}

fn part1(input: &str) {
  let notes = parse_input(input);

  let valid_values: HashSet<i32> = notes.fields
    .iter()
    .flat_map(|(_, set)| set)
    .cloned()
    .collect();


  let invalid_sum: i32 = notes.nearby_tickets
    .iter()
    .flatten()
    .filter(|value| !valid_values.contains(value))
    .sum();

  println!("{:?}", invalid_sum);
}

fn part2(input: &str) {
  let mut notes = parse_input(input);

  let valid_values: HashSet<i32> = notes.fields
    .iter()
    .flat_map(|(_, set)| set)
    .cloned()
    .collect();

  notes.nearby_tickets = notes.nearby_tickets
    .iter()
    .filter(|ticket|
      ticket.iter().all(|value| valid_values.contains(value))
    )
    .cloned()
    .collect();

  let tickets_by_index: Vec<HashSet<i32>> = (0..notes.ticket.len()).map(|index| {
    let mut set: HashSet<i32> = notes.nearby_tickets.iter().map(|v| v[index]).collect();
    set.insert(notes.ticket[index]);
    set
  }).collect();

  let mut order: HashMap<usize, String> = HashMap::new();
  let mut indices: VecDeque<usize> = (0..notes.ticket.len()).collect();

  while let Some(index) = indices.pop_front() {
    let ticket_set = &tickets_by_index[index];
    let match_count = notes.fields.iter().filter(|(_, field_set)| {
      ticket_set.is_subset(field_set)
    }).count();

    if match_count == 1 {
      let field = notes.fields.iter().find(|(_, field_set)| {
        ticket_set.is_subset(field_set)
      }).unwrap();
      order.insert(index, field.0.clone());
      notes.fields.remove(&field.0.clone());
    } else {
      indices.push_back(index);
    }
  }

  let product: i64 = order
    .iter()
    .filter(|(_, name)| name.contains("departure"))
    .fold(1, |product, (index, _)| product * notes.ticket[*index] as i64);

  println!("{:?}", product);
}