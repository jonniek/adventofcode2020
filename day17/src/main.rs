use std::io::{self, Read};
use std::collections::HashMap;
use std::cmp::{max, min};

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  part1(&input);
  part2(&input);
}

type Coordinate = (i32, i32, i32);
type Coordinate2 = (i32, i32, i32, i32);
type Cubes = HashMap<Coordinate, bool>;
type Cubes2 = HashMap<Coordinate2, bool>;

fn parse_input(input: &str) -> Cubes {
  input.lines().enumerate().flat_map(|(y, line)| {
    line.chars().enumerate().map(|(x, ch)| {
      let pos: Coordinate = (x as i32, y as i32, 0);
      match ch {
        '#' => (pos, true),
        _ => (pos, false),
      }
    }).collect::<Vec<(Coordinate, bool)>>()

  }).collect()
}

fn parse_input2(input: &str) -> Cubes2 {
  input.lines().enumerate().flat_map(|(y, line)| {
    line.chars().enumerate().map(|(x, ch)| {
      let pos: Coordinate2 = (x as i32, y as i32, 0, 0);
      match ch {
        '#' => (pos, true),
        _ => (pos, false),
      }
    }).collect::<Vec<(Coordinate2, bool)>>()

  }).collect()
}

fn count_active_neighbours(pos: &Coordinate, cubes: &Cubes) -> i32 {
  let mut count = 0;
  for z in pos.2-1..=pos.2+1 {
    for y in pos.1-1..=pos.1+1  {
      for x in pos.0-1..=pos.0+1 {
        let neighbour_pos: Coordinate = (x as i32, y as i32, z);
        if *pos != neighbour_pos {
          match cubes.get(&neighbour_pos) {
            Some(value) => if *value {
              count += 1;
            },
            None => ()
          }
        }
      }
    }
  }

  count
}

fn count_active_neighbours2(pos: &Coordinate2, cubes: &Cubes2) -> i32 {
  let mut count = 0;
  for w in pos.3-1..=pos.3+1 {
    for z in pos.2-1..=pos.2+1 {
      for y in pos.1-1..=pos.1+1  {
        for x in pos.0-1..=pos.0+1 {
          let neighbour_pos: Coordinate2 = (x as i32, y as i32, z, w);
          if *pos != neighbour_pos {
            match cubes.get(&neighbour_pos) {
              Some(value) => if *value {
                count += 1;
              },
              None => ()
            }
          }
        }
      }
    }
  }

  count
}


fn part1(input: &str) {
  let mut cubes = parse_input(input);
  let mut cubes_clone = cubes.clone();

  let mut min_y: i32 = -1;
  let mut max_y: i32 = input.lines().next().unwrap().len() as i32 + 1;
  let mut min_x: i32 = -1;
  let mut max_x: i32 = max_y;
  let mut min_z: i32 = -1;
  let mut max_z: i32 = 1;

  for _ in 1..=6 {
    for z in min_z..=max_z {
      for y in min_y..=max_y {
        for x in min_x..=max_x {
          let pos: Coordinate = (x, y, z);
          let neighbour_count = count_active_neighbours(&pos, &cubes);
          let state: bool = *cubes.get(&pos).unwrap_or(&false);
          let next_state = match (state, neighbour_count) {
            (true, 2) => true,
            (true, 3) => true,
            (false, 3) => true,
            _ => false,
          };

          if next_state {
            min_x = min(min_x, pos.0 - 1);
            min_y = min(min_y, pos.1 - 1);
            min_z = min(min_z, pos.2 - 1);

            max_x = max(max_x, pos.0 + 1);
            max_y = max(max_y, pos.1 + 1);
            max_z = max(max_z, pos.2 + 1);
          }

          cubes_clone.insert(pos, next_state);
        }
      }
    }

    cubes = cubes_clone.clone();
  }

  let active = cubes.values().filter(|val| **val).count();
  println!("{:?}", active);
}

fn part2(input: &str) {
  let mut cubes = parse_input2(input);
  let mut cubes_clone = cubes.clone();

  let mut min_y: i32 = -1;
  let mut max_y: i32 = input.lines().next().unwrap().len() as i32 + 1;
  let mut min_x: i32 = min_y;
  let mut max_x: i32 = max_y;
  let mut min_z: i32 = min_y;
  let mut max_z: i32 = 1;
  let mut min_w: i32 = min_y;
  let mut max_w: i32 = max_y;

  for _ in 1..=6 {

    for w in min_w..=max_w {
      for z in min_z..=max_z {
        for y in min_y..=max_y {
          for x in min_x..=max_x {

            let pos: Coordinate2 = (x, y, z, w);
            let neighbour_count = count_active_neighbours2(&pos, &cubes);
            let state: bool = *cubes.get(&pos).unwrap_or(&false);
            let next_state = match (state, neighbour_count) {
              (true, 2) => true,
              (true, 3) => true,
              (false, 3) => true,
              _ => false,
            };

            if next_state {
              min_x = min(min_x, pos.0 - 1);
              min_y = min(min_y, pos.1 - 1);
              min_z = min(min_z, pos.2 - 1);
              min_w = min(min_w, pos.3 - 1);

              max_x = max(max_x, pos.0 + 1);
              max_y = max(max_y, pos.1 + 1);
              max_z = max(max_z, pos.2 + 1);
              max_w = max(max_w, pos.3 + 1);
            }

            cubes_clone.insert(pos, next_state);
          }
        }
      }
    }

    cubes = cubes_clone.clone();
  }

  let active = cubes.values().filter(|val| **val).count();
  println!("{:?}", active);
}