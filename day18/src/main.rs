use std::io::{self, Read};
use std::collections::VecDeque;

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  part1(&input);
  part2(&input);
}

#[derive(PartialEq,Debug)]
enum InfixBracket {
  Open,
  Close
}

#[derive(PartialEq,Debug)]
enum InfixSymbol {
  Product,
  Sum,
  Bracket(InfixBracket),
  Value(String),
}

fn infix_symbol_to_string(symbol: InfixSymbol) -> String {
  match symbol {
    InfixSymbol::Product => "*".to_string(),
    InfixSymbol::Sum => "+".to_string(),
    InfixSymbol::Value(value) => value,
    _ => panic!("Bracket to string not implemented")
  }
}

fn infix_to_postfix(input: &str) -> String {
  let input_fixed = input.replace("(", "( ").replace(")", " )");

  let mut stack: VecDeque<InfixSymbol> = VecDeque::new();

  let mut output: Vec<String> = vec!();

  for thing in input_fixed.split(" ") {
    let symbol = match thing {
      "+" => InfixSymbol::Sum,
      "*" => InfixSymbol::Product,
      "(" => InfixSymbol::Bracket(InfixBracket::Open),
      ")" => InfixSymbol::Bracket(InfixBracket::Close),
      num => InfixSymbol::Value(num.to_string()),
    };

    match symbol {
      InfixSymbol::Bracket(InfixBracket::Close) => {
        while let Some(symbol) = stack.pop_back() {
          if symbol == InfixSymbol::Bracket(InfixBracket::Open) {
            break;
          }
          let symbol_string = infix_symbol_to_string(symbol);
          output.push(symbol_string)
        }
      },
      InfixSymbol::Bracket(bracket) => stack.push_back(InfixSymbol::Bracket(bracket)),
      InfixSymbol::Value(value) => output.push(value),
      operand => {
        let prev = stack.iter().last();

        let prev_is_sum = prev == Some(&InfixSymbol::Sum);
        let prev_is_product = prev == Some(&InfixSymbol::Product);

        if prev_is_sum || prev_is_product {
          let stack_symbol = stack.pop_back().unwrap();
          let symbol_string = infix_symbol_to_string(stack_symbol);
          output.push(symbol_string);
        }
        stack.push_back(operand);
      },
    }
  }

  while let Some(symbol) = stack.pop_back() {
    let symbol_string = infix_symbol_to_string(symbol);
    output.push(symbol_string);
  }

  output.join(" ")
}

enum PostFixOperand {
  Sum,
  Product,
}

enum PostFixSymbol {
  Operand(PostFixOperand),
  Value(i64),
}

fn calculate_postfix(input: &str) -> i64 {
  let mut stack: VecDeque<i64> = VecDeque::new();

  for thing in input.split(" ") {

    let symbol = match thing {
      "+" => PostFixSymbol::Operand(PostFixOperand::Sum),
      "*" => PostFixSymbol::Operand(PostFixOperand::Product),
      num => PostFixSymbol::Value(num.parse::<i64>().unwrap()),
    };

    match symbol {
      PostFixSymbol::Operand(op) => {
        let b = stack.pop_back().unwrap();
        let a = stack.pop_back().unwrap();
        let result = match op {
          PostFixOperand::Sum => a + b,
          PostFixOperand::Product => a * b,
        };
        stack.push_back(result);
      },
      PostFixSymbol::Value(n) => stack.push_back(n),
    }
  }

  stack.pop_back().unwrap_or(0)
}

fn compute(input: &str) -> i64 {
  let postfix = infix_to_postfix(input);
  calculate_postfix(&postfix)
}

fn part1(input: &str) {
  let sum: i64 = input.lines().map(|line| compute(line)).sum();
  println!("{}", sum);
}

fn compute2(input: &str) -> i64 {
  let postfix = infix_to_postfix2(input);
  calculate_postfix(&postfix)
}

fn infix_to_postfix2(input: &str) -> String {
  let input_fixed = input.replace("(", "( ").replace(")", " )");

  let mut stack: VecDeque<InfixSymbol> = VecDeque::new();

  let mut output: Vec<String> = vec!();

  for thing in input_fixed.split(" ") {
    let symbol = match thing {
      "+" => InfixSymbol::Sum,
      "*" => InfixSymbol::Product,
      "(" => InfixSymbol::Bracket(InfixBracket::Open),
      ")" => InfixSymbol::Bracket(InfixBracket::Close),
      num => InfixSymbol::Value(num.to_string()),
    };

    match symbol {
      InfixSymbol::Bracket(InfixBracket::Close) => {
        while let Some(symbol) = stack.pop_back() {
          if symbol == InfixSymbol::Bracket(InfixBracket::Open) {
            break;
          }
          let symbol_string = infix_symbol_to_string(symbol);
          output.push(symbol_string)
        }
      },
      InfixSymbol::Bracket(bracket) => stack.push_back(InfixSymbol::Bracket(bracket)),
      InfixSymbol::Value(value) => output.push(value),
      InfixSymbol::Product => {
        let prev = stack.iter().last();
        let prev_is_sum = prev == Some(&InfixSymbol::Sum);
        let prev_is_product = prev == Some(&InfixSymbol::Product);

        if prev_is_sum || prev_is_product {
          let stack_symbol = stack.pop_back().unwrap();
          let symbol_string = infix_symbol_to_string(stack_symbol);
          output.push(symbol_string);
        }
        stack.push_back(InfixSymbol::Product);
      },
      InfixSymbol::Sum => {
        let prev = stack.iter().last();
        let prev_is_sum = prev == Some(&InfixSymbol::Sum);

        if prev_is_sum {
          let stack_symbol = stack.pop_back().unwrap();
          let symbol_string = infix_symbol_to_string(stack_symbol);
          output.push(symbol_string);
        }
        stack.push_back(InfixSymbol::Sum);
      }
    }
  }

  while let Some(symbol) = stack.pop_back() {
    let symbol_string = infix_symbol_to_string(symbol);
    output.push(symbol_string);
  }

  output.join(" ")
}

fn part2(input: &str) {
  let sum: i64 = input.lines().map(|line| compute2(line)).sum();
  println!("{}", sum);
}


#[test]
fn test_calculate_postfix() {
  let input = "10 2 8 * +";
  let result = calculate_postfix(input);
  assert_eq!(result, 26);
}

#[test]
fn test_infix_to_postfix() {
  let input = "10 + 2 * 8";
  let result = infix_to_postfix(input);
  assert_eq!(result, "10 2 + 8 *");

  let input = "(10 + 2) * 8";
  let result = infix_to_postfix(input);
  assert_eq!(result, "10 2 + 8 *");


  let input = "(10 + (2 * 2)) * 8";
  let result = infix_to_postfix(input);
  assert_eq!(result, "10 2 2 * + 8 *");


  let input = "(10 + 10 * (2 * 2)) * 8";
  let result = infix_to_postfix(input);
  assert_eq!(result, "10 10 + 2 2 * * 8 *");
}

#[test]
fn test_compute() {
  let input = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
  let result = compute(input);
  assert_eq!(result, 12240);

  let input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
  let result = compute(input);
  assert_eq!(result, 13632);
}

#[test]
fn test_compute2() {
  let input = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
  let result = compute2(input);
  assert_eq!(result, 1445);

  let input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
  let result = compute2(input);
  assert_eq!(result, 23340);
}
