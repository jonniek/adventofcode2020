use std::io::{self, Read};
use std::collections::HashMap;

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  part1(&input);
  part2(&input);
}

type Passport = HashMap<String, String>;


fn validator(field: &str, value: &str) -> bool {
  let is_valid = match field {
    "byr" => {
      let result = value.parse::<i32>();
      if result.is_err() { return false; }
      let year = result.unwrap();

      if year < 1920 || year > 2002 {
        return false;
      }

      true
    },
    "iyr" => {
      let result = value.parse::<i32>();
      if result.is_err() { return false; }
      let year = result.unwrap();

      if year < 2010 || year > 2020 {
        return false;
      }

      true
    },
    "eyr" => {
      let result = value.parse::<i32>();
      if result.is_err() { return false; }
      let year = result.unwrap();

      if year < 2020 || year > 2030 {
        return false;
      }

      true
    },
    "hgt" => {
      let (number, unit) = value.split_at(value.len() - 2);
      if unit != "cm" && unit.trim() != "in" {
        return false;
      }

      let length_result: Result<i32, _> = number.parse::<i32>();
      if length_result.is_err() { return false; }


      let length = length_result.unwrap();
      if unit == "cm" && (length < 150 || length > 193) {
        return false;
      }

      if unit == "in" && (length < 59 || length > 76) {
        return false
      }

      true
    },
    "hcl" => {
      let (hash, code) = value.split_at(1);
      if hash != "#" { return false }
      code.len() == 6 && code.chars().all(|ch| ch.is_digit(16))
    }
    "ecl" => vec!("amb","blu","brn","gry","grn","hzl","oth").contains(&value),
    "pid" => value.chars().all(|ch| ch.is_digit(10)) && value.len() == 9,
    "cid" => true,
    _ => false,
  };
  is_valid
}

fn is_valid_passport(passport: &Passport) -> bool {
  let required_fields: Vec<&str> = vec!("byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid");
  let missing_count = required_fields.iter().filter(|field| !passport.contains_key(**field)).count();
  missing_count == 0
}

fn is_valid_passport_2(passport: &Passport) -> bool {
  let required_fields: Vec<&str> = vec!("byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid");
  let invalid_count = required_fields.iter().filter(|field| {
    let value_option = passport.get(**field);
    if value_option.is_none() { return true }

    let value = value_option.unwrap();
    !validator(field, value)
  }).count();
  invalid_count == 0
}

fn parse_passports(input: &str) -> Vec<Passport> {
  input
    .split("\r\n\r\n")
    .map(|passport| {
      let fields: Vec<(String, String)> = passport.replace("\r\n", " ").split(" ").map(|field| {
        let mut fieldsplit = field.split(":");
        let key = fieldsplit.next().unwrap();
        let value = fieldsplit.next().unwrap();
        return (key.to_string(), value.to_string())
      }).collect();

      let map: Passport = fields.iter().cloned().collect();
      map
    })
    .collect()
}

fn part1(input: &str) {
  let passports = parse_passports(input);
  let valid_passports = passports.iter().filter(|pass| is_valid_passport(pass)).count();
  println!("{}", valid_passports);
}


fn part2(input: &str) {
  let passports = parse_passports(input);
  let valid_passports = passports.iter().filter(|pass| is_valid_passport_2(pass)).count();
  println!("{}", valid_passports);
}
