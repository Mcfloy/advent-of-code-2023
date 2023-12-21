use std::fs;

// Code heavily inspired by maneatingape
// https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2023/day12.rs

// First part is the broken records, second part is the expected number of springs
type Input = Vec<(Vec<char>, Vec<usize>)>;

pub fn parse(path: &str) -> Input {
  let content = fs::read_to_string(path).unwrap();
  let input = content.lines()
    .map(|line| {
      let (prefix, suffix) = line.split_once(' ').unwrap();
      let first: Vec<char> = prefix.chars().collect();
      let second: Vec<usize> = suffix.split(',').map(|s| s.parse().unwrap()).collect();
      (first, second)
    })
    .collect();

  input
}

pub fn part1(path: &str) -> u64 {
  solve(&parse(path), 1)
}

pub fn part2(path: &str) -> u64 {
  solve(&parse(path), 5)
}

pub fn solve(input: &Input, repeat: usize) -> u64 {
  let mut result = 0;
  let mut pattern: Vec<char> = Vec::new();
  let mut springs: Vec<usize> = Vec::new();

  // Prefill tables with some margin as suggested by maneatingape
  let mut broken = vec![0; 200];
  let mut table = vec![0; 200 * 50];

  for (first, second) in input {
    pattern.clear();
    springs.clear();

    for _ in 1..repeat {
      pattern.extend_from_slice(first);
      pattern.push('?');
      springs.extend_from_slice(second);
    }

    // Trailing '.' to ignore check bounds
    pattern.extend_from_slice(first);
    pattern.push('.');
    springs.extend_from_slice(second);

    let mut sum = 0;
    broken.push(0);

    for (i, &b) in pattern.iter().enumerate() {
      if b != '.' {
        sum += 1;
      }
      broken[i + 1] = sum;
    }

    let margin = pattern.len() - springs.iter().sum::<usize>() - springs.len() + 1;

    let size = springs[0];
    let mut sum = 0;
    let mut valid = true;

    for i in 0..margin {
      if pattern[i + size] == '#' {
        sum = 0;
      } else if valid && broken[i + size] - broken[i] == size {
        sum += 1;
      }

      table[i + size] = sum;

      valid &= pattern[i] != '#';
    }

    let mut start = size + 1;

    for (row, &size) in springs.iter().enumerate().skip(1) {
      let previous = (row - 1) * pattern.len();
      let current = row * pattern.len();

      sum = 0;

      for i in start..(start + margin) {
        if pattern[i + size] == '#' {
          sum = 0;
        } else if table[previous + i - 1] > 0 && pattern[i - 1] != '#' && broken[i + size] - broken[i] == size {
          sum += table[previous + i - 1];
        }

        table[current + i + size] = sum;
      }

      start += size + 1;
    }
    result += sum;
  }
  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part1() {
    assert_eq!(part1("src/day12/test-part1.txt"), 21);
  }

  #[test]
  fn test_part2() {
    assert_eq!(part2("src/day12/test-part2.txt"), 525152);
  }
}