use std::fs;

pub fn part1(path: &str) -> u64 {
  let content = fs::read_to_string(path).expect("Unable to read file");

  let mut lines = content.lines();
  let times: Vec<u64> = lines.next().unwrap().replace("Time: ", "")
    .split_whitespace()
    .map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
  let distances: Vec<u64> = lines.next().unwrap().replace("Distance: ", "")
    .split_whitespace()
    .map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();

  let mut result: u64 = 0;
  for (i, time) in times.iter().enumerate() {
    let mut times_with_record = 0;
    for time_holding in 1..=*time - 1 {
      let speed = time_holding;
      let remaining_time = time - time_holding;
      let distance = speed * remaining_time;
      if distance > distances[i] {
        times_with_record += 1;
      }
    }
    if result == 0 {
      result = times_with_record;
    } else {
      result *= times_with_record;
    }
  }

  result
}

pub fn part2(path: &str) -> u64 {
  let content = fs::read_to_string(path).expect("Unable to read file");

  let mut lines = content.lines();
  let time: u64 = lines.next().unwrap().replace("Time: ", "")
    .replace(" ", "")
    .parse::<u64>().unwrap();
  let distance = lines.next().unwrap().replace("Distance: ", "")
    .replace(" ", "")
    .parse::<u64>().unwrap();

  let mut times_with_record = 0;
  for time_holding in 1..=time - 1 {
    let speed = time_holding;
    let remaining_time = time - time_holding;
    let attempted_distance = speed * remaining_time;
    if attempted_distance > distance {
      times_with_record += 1;
    }
  }

  times_with_record
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part1() {
    assert_eq!(part1("src/day6/test-part1.txt"), 288);
  }

  #[test]
  fn test_part2() {
    assert_eq!(part2("src/day6/test-part2.txt"), 71503);
  }
}