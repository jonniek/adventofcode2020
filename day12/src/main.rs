use std::io::{self, Read};

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  part1(&input);
  part2(&input);
}

#[derive(Debug)]
enum Action {
  N(i32),
  S(i32),
  E(i32),
  W(i32),
  L(i32),
  R(i32),
  F(i32),
}

fn parse_actions(input: &str) -> Vec<Action> {
  input.lines().map(|line| {
    let (ch, val) = line.split_at(1);
    let value = val.parse::<i32>().unwrap();
    match ch {
      "N" => Action::N(value),
      "S" => Action::S(value),
      "E" => Action::E(value),
      "W" => Action::W(value),
      "L" => Action::L(value),
      "R" => Action::R(value),
      "F" => Action::F(value),
      _ => panic!("Unexpected char {}", ch),
    }
  }).collect()
}

const DIRECTIONS: [(i32, i32); 4] = [
  (0, 1),
  (-1, 0),
  (0, -1),
  (1, 0)
];


#[derive(Debug)]
struct Position {
  loc: (i32, i32),
  dir: (i32, i32),
}

fn manhattan_distance(pos: (i32, i32)) -> i32 {
  pos.0.abs() + pos.1.abs()
}

fn rotate(dir: (i32, i32), action: &Action) -> (i32, i32) {
  let rotation: i32 = match action {
    Action::R(n) => *n,
    Action::L(n) => 360 - n,
    _ => panic!("Unexpected rotation {:?}", action),
  };

  let index = DIRECTIONS.iter().position(|&r| r == dir).unwrap();
  let rotation_count: usize = (rotation / 90).abs() as usize;

  *DIRECTIONS.iter().cycle().skip(index + rotation_count).next().unwrap()
}

fn part1(input: &str) {
  let actions: Vec<Action> = parse_actions(input);

  let mut pos = Position {
    loc: (0, 0),
    dir: (0, 1),
  };

  for action in actions.iter() {
    match action {
      Action::L(_) => pos.dir = rotate(pos.dir, &action),
      Action::R(_) => pos.dir = rotate(pos.dir, &action),
      Action::F(n) => pos.loc = (
        pos.loc.0 + n * pos.dir.0,
        pos.loc.1 + n * pos.dir.1,
      ),
      Action::N(n) => pos.loc = (
        pos.loc.0 + n,
        pos.loc.1,
      ),
      Action::S(n) => pos.loc = (
        pos.loc.0 - n,
        pos.loc.1,
      ),
      Action::E(n) => pos.loc = (
        pos.loc.0,
        pos.loc.1 + n,
      ),
      Action::W(n) => pos.loc = (
        pos.loc.0,
        pos.loc.1 - n,
      ),
    };
  }

  println!("{:?}", manhattan_distance(pos.loc));
}

fn rotate_vector(dir: (i32, i32), vector: (i32, i32), degrees: i32,) -> (i32, i32) {
  let count = degrees / 90;
  let mut res = dir;
  for _ in 0..count {
    res = (res.1 * vector.0, res.0 * vector.1);
  }
  res
}

fn rotate2(waypoint: (i32, i32), action: &Action) -> (i32, i32) {
  let left = (1, -1);
  let right = (-1, 1);
  match action {
    Action::R(n) => rotate_vector(waypoint, right, n / 90),
    Action::L(n) => rotate_vector(waypoint, left, n / 90),
    _ => panic!("Unexpected rotation {:?}", action),
  }
}

fn part2(input: &str) {
  let actions: Vec<Action> = parse_actions(input);

  let mut pos = (0, 0);
  let mut waypoint = (1, 10);

  for action in actions.iter() {

    match action {
      Action::L(_) => waypoint = rotate2(waypoint, &action),
      Action::R(_) => waypoint = rotate2(waypoint, &action),
      Action::F(n) => pos = (
        pos.0 + n * waypoint.0,
        pos.1 + n * waypoint.1,
      ),
      Action::N(n) => waypoint = (
        waypoint.0 + n,
        waypoint.1,
      ),
      Action::S(n) => waypoint = (
        waypoint.0 - n,
        waypoint.1,
      ),
      Action::E(n) => waypoint = (
        waypoint.0,
        waypoint.1 + n,
      ),
      Action::W(n) => waypoint = (
        waypoint.0,
        waypoint.1 - n,
      ),
    };
  }

  println!("{:?}", manhattan_distance(pos));
}