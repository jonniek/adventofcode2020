use std::io::{self, Read};
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp::max;
use std::collections::BTreeSet;

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  part1(&input);
  part2(&input);
}

fn part1(input: &str) {
  let adapters: HashSet<i64> = input.lines().map(|line| line.parse::<i64>().unwrap()).collect();
  let max = adapters.iter().max().unwrap();

  let mut jolts: HashMap<i64, i64> = [(3, 1)].iter().cloned().collect();
  let mut jump = 0;
  for i in 1..=*max {
    jump += 1;

    if adapters.contains(&i) {
      let count = jolts.entry(jump).or_insert(0);
      *count += 1;
      jump = 0;
    }
  }

  let one_jolt = jolts.get(&1).unwrap();
  let three_jolt = jolts.get(&3).unwrap();

  println!("{:?}", one_jolt * three_jolt);
}

fn part2(input: &str) {
  let adapters: BTreeSet<i64> = input.lines().map(|line| line.parse::<i64>().unwrap()).collect();

  let mut cache: HashMap<i64, i64> = HashMap::new();
  for n in adapters.iter().rev()  {
    let result = traverse(*n, &adapters, &cache);
    cache.insert(*n, result);
  }

  let result = traverse(0, &adapters, &cache);
  println!("{}", result);
}

fn traverse(n: i64, adapters: &BTreeSet<i64>, cache: &HashMap<i64, i64>) -> i64 {
  match cache.get(&n) {
    Some(value) => *value,
    None => {
      max(
        1,
        (n + 1..n + 4).into_iter().filter(|i| adapters.contains(&i)).collect::<Vec<i64>>()
          .iter()
          .map(|child| traverse(*child, adapters, cache)).sum::<i64>()
      )
    }
  }
}