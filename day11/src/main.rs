use std::io::{self, Read};
use std::collections::HashMap;

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  part1(&input);
  part2(&input);
}

#[derive(Debug,PartialEq,Clone,Copy)]
enum SeatType {
  Full,
  Empty,
  Floor
}

type Coord = (i32, i32);
type Space = HashMap<Coord, SeatType>;

const ADJACENT_COORDS: [Coord; 8] = [
  (1, 1),
  (1, -1),
  (1, 0),

  (-1, 1),
  (-1, -1),
  (-1, 0),

  (0, 1),
  (0, -1),
];

fn adjacent_seat_count(pos: Coord, space: &Space) -> usize {
  let adjacent_coords: Vec<Coord> = ADJACENT_COORDS.iter().map(|(y, x)| (pos.0 + y, pos.1 + x)).collect();

  adjacent_coords
    .iter()
    .map(|coord| {
      space.get(coord).unwrap_or(&SeatType::Floor)
    })
    .filter(|asd| *asd == &SeatType::Full)
    .count()
}

fn find_visible_seat(pos: Coord, dir: Coord, space: &Space) -> Option<&SeatType> {
  let mut distance = 1;
  let mut seat: Option<&SeatType> = space.get(&(pos.0 + dir.0, pos.1 + dir.1));

  while seat == Some(&SeatType::Floor) {
    distance += 1;
    seat = space.get(&(
      pos.0 + dir.0 * distance,
      pos.1 + dir.1 * distance
    ));
  }

  seat
}

fn find_visible_adjacent_seat_count(pos: Coord, space: &Space) -> usize {
  ADJACENT_COORDS
    .iter()
    .map(|dir| find_visible_seat(pos, *dir, space))
    .filter(|seat| *seat == Some(&SeatType::Full))
    .count()
}

fn part1(input: &str) {
  let mut space: Space= input.lines().enumerate().flat_map(|(y, line)| {
    line.chars().enumerate().map(|(x, char)| {
      let seat = match char {
        'L' => SeatType::Empty,
        '#' => SeatType::Full,
        _ => SeatType::Floor,
      };
      ((y as i32, x as i32), seat)
    }).collect::<Vec<(Coord, SeatType)>>()
  }).collect();

  loop {
    let space_clone = space.clone();

    for (pos, seat) in space.iter_mut() {
      let adjacent_taken = adjacent_seat_count(*pos, &space_clone);
      *seat = match (*seat, adjacent_taken) {
        (SeatType::Empty, 0) => SeatType::Full,
        (SeatType::Full, n) => if n > 3 { SeatType::Empty } else { SeatType::Full },
        _ => SeatType::Floor,
      };
    }

    if space_clone == space {
      break;
    }
  }

  let occupied = space.iter().filter(|(_, seat)| *seat == &SeatType::Full).count();
  println!("{}", occupied);
}

fn part2(input: &str) {
  let mut space: Space= input.lines().enumerate().flat_map(|(y, line)| {
    line.chars().enumerate().map(|(x, char)| {
      let seat = match char {
        'L' => SeatType::Empty,
        '#' => SeatType::Full,
        _ => SeatType::Floor,
      };
      ((y as i32, x as i32), seat)
    }).collect::<Vec<(Coord, SeatType)>>()
  }).collect();

  loop {
    let space_clone = space.clone();

    for (pos, seat) in space.iter_mut() {
      let adjacent_taken = find_visible_adjacent_seat_count(*pos, &space_clone);
      *seat = match (*seat, adjacent_taken) {
        (SeatType::Empty, 0) => SeatType::Full,
        (SeatType::Full, n) => if n > 4 { SeatType::Empty } else { SeatType::Full },
        _ => *seat,
      };
    }

    if space_clone == space {
      break;
    }
  }

  let occupied = space.iter().filter(|(_, seat)| *seat == &SeatType::Full).count();
  println!("{}", occupied);
}