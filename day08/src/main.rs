use std::io::{self, Read};
use std::collections::HashSet;

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  part1(&input);
  part2(&input);
}

#[derive(Debug,Clone)]
enum Operation {
  ACC(i32),
  JMP(i32),
  NOP(i32),
}

#[derive(PartialEq)]
enum Sigterm {
  LOOP,
  EOF
}

fn operate(operations: &Vec<Operation>) -> (i32, Sigterm)  {
  let mut accumulator = 0;

  let mut visited: HashSet<i32> = HashSet::new();
  let mut index: i32 = 0;

  while !visited.contains(&index)  {
    visited.insert(index);

    if index < 0 {
      panic!("Unexpected negative index {}", index);
    }

    if index > operations.len() as i32 - 1 {
      return (accumulator, Sigterm::EOF)
    }

    match &operations[index as usize] {
      Operation::ACC(value) => {
        accumulator += value;
        index += 1;
      },
      Operation::JMP(value) => index += value,
      Operation::NOP(_) => index += 1,
    }
  }
  return (accumulator, Sigterm::LOOP);
}

fn parse_operations(input: &str) -> Vec<Operation> {
  input.lines().map(|line| {
    let mut split = line.split(" ");
    let op = split.next().unwrap();
    let value = split.next().unwrap().parse::<i32>().unwrap();

    let operation = match op {
      "nop" => Operation::NOP(value),
      "acc" => Operation::ACC(value),
      "jmp" => Operation::JMP(value),
      _ => panic!("Unexpected operation code {}", op),
    };

    operation
  }).collect()
}

fn part1(input: &str) {
  let operations = parse_operations(input);
  let (result, _)= operate(&operations);
  println!("{:?}", result);
}

fn flip_operation(operation: &Operation) -> Operation {
  match operation {
    Operation::ACC(_) => panic!("Cannot flip ACC operation code"),
    Operation::JMP(value) => Operation::NOP(*value),
    Operation::NOP(value) => Operation::JMP(*value),
  }
}

fn part2(input: &str) {
  let operations = parse_operations(input);
  for (index, operation) in operations.iter().enumerate() {
    match operation {
      Operation::ACC(_) => continue,
      _ => {
        let mut cloned_operations = operations.clone();
        cloned_operations[index] = flip_operation(operation);
        let (result, term_code) = operate(&cloned_operations);
        if term_code == Sigterm::EOF {
          println!("{:?}", result);
        }
      }
    }
  }
}
