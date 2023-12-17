use std::fs;

#[derive(Clone, Debug)]
struct Sequence {
  numbers: Vec<i64>
}

impl Sequence {
  pub fn new() -> Self {
    Sequence {
      numbers: vec![],
    }
  }

  pub fn generate_sequence(&self) -> Sequence {
    let mut sequence = Sequence::new();
    if self.numbers.len() < 2 {
      return sequence;
    }
    for i in 1..self.numbers.len() {
      sequence.numbers.push(self.numbers[i] - self.numbers[i - 1]);
    }
    sequence
  }

  pub fn contains_all_zeros(&self) -> bool {
    self.numbers.iter().all(|&x| x == 0)
  }

  pub fn extrapolate_forward(&self, sequence: Sequence) -> Sequence {
    let mut new_sequence = Sequence::new();
    for i in 0..self.numbers.len() - 1 {
      new_sequence.numbers.push(self.numbers[i] + sequence.numbers[i]);
    }
    new_sequence.numbers.push(self.numbers.last().unwrap() + sequence.numbers.last().unwrap());
    new_sequence
  }

  pub fn extrapolate_backward(&self, sequence: Sequence) -> Sequence {
    let mut new_sequence = Sequence::new();
    for i in 0..self.numbers.len() - 1 {
      new_sequence.numbers.push(self.numbers[i] - sequence.numbers[i]);
    }
    new_sequence.numbers.reverse();
    new_sequence.numbers.push(self.numbers.first().unwrap() - sequence.numbers.first().unwrap());
    new_sequence.numbers.reverse();
    new_sequence
  }
}

pub fn part1(path: &str) -> i64 {
  let content = fs::read_to_string(path).expect("Unable to read file");

  let mut result = 0;
  for line in content.lines() {
    let mut sequence = Sequence::new();
    for number in line.split(" ") {
      sequence.numbers.push(number.parse::<i64>().unwrap());
    }
    let mut sequences: Vec<Sequence> = vec![];
    sequences.push(sequence);
    while !sequences.last().unwrap().contains_all_zeros() {
      sequences.push(sequences.last().unwrap().generate_sequence());
    }
    sequences.reverse();
    for i in 0..sequences.len() - 1 {
      sequences[i + 1] = sequences[i + 1].extrapolate_forward(sequences[i].clone());
    }
    result += sequences.last().unwrap().numbers.last().unwrap();
  }

  result
}

pub fn part2(path: &str) -> i64 {
  let content = fs::read_to_string(path).expect("Unable to read file");

  let mut result = 0;
  for line in content.lines() {
    let mut sequence = Sequence::new();
    for number in line.split(" ") {
      sequence.numbers.push(number.parse::<i64>().unwrap());
    }
    let mut sequences: Vec<Sequence> = vec![];
    sequences.push(sequence);
    while !sequences.last().unwrap().contains_all_zeros() {
      sequences.push(sequences.last().unwrap().generate_sequence());
    }
    sequences.reverse();
    for i in 0..sequences.len() - 1 {
      sequences[i + 1] = sequences[i + 1].extrapolate_backward(sequences[i].clone());
    }
    result += sequences.last().unwrap().numbers.first().unwrap();
  }

  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part1() {
    assert_eq!(part1("src/day9/test-part1.txt"), 114);
  }

  #[test]
  fn test_part2() {
    assert_eq!(part2("src/day9/test-part2.txt"), 2);
  }
}