use std::io::{self, Read};

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  part1(&input);
  part2(&input);
}

#[derive(Debug)]
struct Rule {
  min: i32,
  max: i32,
  rule: char,
  password: String,
}

fn is_valid_password(rule: &Rule) -> bool {
  let rule_count: i32 = rule.password.chars().fold(0, |count, c| {
    if c == rule.rule {
      return count + 1
    }
    count
  });

  if rule_count >= rule.min && rule_count <= rule.max {
    return true
  }

  false
}

fn get_rules(input: &str) -> Vec<Rule> {
  input
    .lines()
    .map(|v| {
      let splits: Vec<String> = v.split(" ").map(|b| b.to_string()).collect();
      let minmax: Vec<i32> = splits[0].split("-").map(|v| v.parse::<i32>().unwrap()).collect();

      Rule {
        min: minmax[0],
        max: minmax[1],
        rule: splits[1].chars().next().unwrap(),
        password: splits[2].clone()
      }
    })
    .collect()
}

fn part1(input: &str) {
  let rules = get_rules(input);
  let valid_count = rules.into_iter().filter(is_valid_password).count();
  println!("{:?}", valid_count);
}

fn is_valid_password2(rule: &Rule) -> bool {
  let password_chars: Vec<char> = rule.password.chars().collect();

  let a = password_chars[(rule.min - 1) as usize] == rule.rule;
  let b = password_chars[(rule.max  - 1) as usize] == rule.rule;

  !(a && b) && (a || b)
}

fn part2(input: &str) {
  let rules = get_rules(input);
  let valid_count = rules.into_iter().filter(is_valid_password2).count();
  println!("{:?}", valid_count);
}
