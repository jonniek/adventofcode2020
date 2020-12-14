use std::io::{self, Read};
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  part1(&input);
  part2(&input);
}

#[derive(Debug)]
enum Operation {
  Mask(String),
  Mem((i64,i64))
}

fn compute_value(ch: char, mask: &str) -> i64 {
  let mut value = 0;
  let base: i64 = 2;
  let mut power: u32 = 0;
  for n in mask.chars().rev() {
    if n == ch {
      value += base.pow(power)
    }
    power += 1;
  }

  value
}

fn mask_value(value: i64, mask: &str) -> i64 {
  let or_mask = compute_value('1', mask);
  let set_bits = compute_value('X', mask);
  (value & set_bits) | or_mask
}

fn parse_operations(input: &str) -> Vec<Operation> {
  input
    .replace(" ", "")
    .replace("mem[", "")
    .replace("]", "")
    .lines()
    .map(|line| {
      let mut split = line.split("=");
      let operand = split.next().unwrap();
      let value = split.next().unwrap();

      if operand == "mask" {
        Operation::Mask(value.to_string())
      } else {
        Operation::Mem((
          operand.parse::<i64>().unwrap(),
          value.parse::<i64>().unwrap()
        ))
      }
    }).collect()
}

fn part1(input: &str) {
  let operations = parse_operations(input);

  let mut mask = "";
  let mut memory: HashMap<i64, i64> = HashMap::new();

  for operation in operations.iter() {
    match operation {
      Operation::Mask(m) => mask = m,
      Operation::Mem((address, value)) => {
        let masked_value = mask_value(*value, mask);
        memory.insert(*address, masked_value);
      }
    }
  }

  let sum: i64 = memory.values().sum();
  println!("{:?}", sum)
}

// nasty recursive sets again
fn find_permutations(number: i64, sum: i64, power: u64, mut perms: HashSet<i64>) -> HashSet<i64> {
  if power > 36 {
    return perms.clone();
  }

  perms.insert(sum);
  let bit = (2 as i64).pow(power as u32);
  let value = number & bit;
  if value != 0 {
    let mut next_perms = find_permutations(number, sum, power + 1, perms.clone());
    perms.insert(sum + value);
    let other = find_permutations(number, sum + value, power + 1, perms);
    next_perms.extend(other);
    return next_perms;
  } else {
    return find_permutations(number, sum, power + 1, perms);
  }
}

fn get_memory_addresses(address: i64, mask: &str) -> HashSet<i64> {
  let masked_ones = compute_value('1', &mask);
  let masked_x = compute_value('X', &mask);

  // a lot of trial and error, pen and paper for this formula
  let base = address | masked_ones;
  let base_value = base ^ (base & masked_x);

  find_permutations(masked_x, 0, 0, HashSet::new()).iter().map(|perm| perm + base_value).collect()
}

fn part2(input: &str) {
  let operations = parse_operations(input);

  let mut mask = "";
  let mut memory: HashMap<i64, i64> = HashMap::new();

  for operation in operations.iter() {
    match operation {
      Operation::Mask(m) => mask = m,
      Operation::Mem((address, value)) => {
        let addresses = get_memory_addresses(*address, mask);
        for address in addresses {
          memory.insert(address, *value);
        }
      }
    }
  }

  let sum: i64 = memory.values().sum();
  println!("{:?}", sum)
}