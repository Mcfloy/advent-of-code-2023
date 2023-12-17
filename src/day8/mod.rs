use std::collections::HashMap;
use std::fs;

use regex::Regex;

enum Instruction {
  Left,
  Right,
}

impl Instruction {
  fn from_line(c: char) -> Instruction {
    match c {
      'L' => Instruction::Left,
      'R' => Instruction::Right,
      _ => panic!("Invalid instruction"),
    }
  }
}

pub fn part1(path: &str) -> u64 {
  let content = fs::read_to_string(path).expect("Unable to read file");

  let mut lines = content.lines();
  let instructions = lines.next().unwrap().chars().map(|x| Instruction::from_line(x)).collect::<Vec<Instruction>>();

  let mut nodes: HashMap<String, (String, String)> = HashMap::new();
  let regex = Regex::new(r"([A-Z]+) = \(([A-Z]+), ([A-Z]+)\)").unwrap();
  for line in lines {
    if line.trim().is_empty() {
      continue;
    }
    let captures = regex.captures(line).unwrap();
    let key = captures.get(1).unwrap().as_str().to_string();
    let left = captures.get(2).unwrap().as_str().to_string();
    let right = captures.get(3).unwrap().as_str().to_string();
    nodes.insert(key, (left, right));
  }

  let mut steps = 0;
  let mut current = String::from("AAA");
  let mut current_node: &(String, String) = nodes.get(&current).unwrap();
  while current != "ZZZ" {
    for instruction in instructions.iter() {
      match instruction {
        Instruction::Left => {
          current = current_node.0.to_string();
          current_node = nodes.get(&current_node.0).unwrap();
          steps += 1;
        }
        Instruction::Right => {
          current = current_node.1.to_string();
          current_node = nodes.get(&current_node.1).unwrap();
          steps += 1;
        }
      }
      if current == "ZZZ" {
        break;
      }
    }
  }

  steps
}

pub fn part2(path: &str) -> u64 {
  let content = fs::read_to_string(path).expect("Unable to read file");

  let mut lines = content.lines();
  let instructions = lines.next().unwrap().chars().map(|x| Instruction::from_line(x)).collect::<Vec<Instruction>>();

  let mut nodes: HashMap<String, (String, String)> = HashMap::new();
  let regex = Regex::new(r"([A-Z1-9]+) = \(([A-Z1-9]+), ([A-Z1-9]+)\)").unwrap();
  for line in lines {
    if line.trim().is_empty() {
      continue;
    }
    let captures = regex.captures(line).unwrap();
    let key = captures.get(1).unwrap().as_str().to_string();
    let left = captures.get(2).unwrap().as_str().to_string();
    let right = captures.get(3).unwrap().as_str().to_string();
    nodes.insert(key, (left, right));
  }

  let mut currents: Vec<String> = vec![];
  let mut current_nodes: Vec<&(String, String)> = vec![];
  let mut steps: Vec<u64> = vec![];
  for key in nodes.keys() {
    if key.ends_with("A") {
      currents.push(key.to_string());
      current_nodes.push(nodes.get(key).unwrap());
      steps.push(0);
    }
  }

  for i in 0..currents.len() {
    while !currents[i].ends_with("Z") {
      for instruction in instructions.iter() {
        match instruction {
          Instruction::Left => {
            currents[i] = current_nodes[i].0.to_string();
            current_nodes[i] = nodes.get(&current_nodes[i].0).unwrap();
            steps[i] += 1;
          }
          Instruction::Right => {
            currents[i] = current_nodes[i].1.to_string();
            current_nodes[i] = nodes.get(&current_nodes[i].1).unwrap();
            steps[i] += 1;
          }
        }
        if currents[i].ends_with("Z") {
          break;
        }
      }
    }
  }

  // Calculate the LCM of the steps.
  // https://en.wikipedia.org/wiki/Least_common_multiple#Using_the_greatest_common_divisor
  let mut lcm = steps[0];
  let gcd = |a: u64, b: u64| -> u64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
      let t = b;
      b = a % b;
      a = t;
    }
    a
  };
  for i in 1..steps.len() {
    lcm = (lcm * steps[i]) / gcd(lcm, steps[i]);
  }

  lcm
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part1() {
    assert_eq!(part1("src/day8/test-part1.txt"), 2);
  }

  #[test]
  fn test_part1b() {
    assert_eq!(part1("src/day8/test-part1b.txt"), 6);
  }

  #[test]
  fn test_part2() {
    assert_eq!(part2("src/day8/test-part2.txt"), 6);
  }
}