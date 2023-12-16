use std::fs;
use std::str::Lines;

use rayon::prelude::*;

// Inspired with the help of aoc32 as the original solution was too slow (brute force)
// I had the idea of using a range map, but would have not thought to use option for seeds and seeds range
// Original link: https://github.com/agent3bood/aoc32/blob/master/src/five/mod.rs
pub struct Almanach {
  seeds: Option<Vec<u64>>,
  seeds_range: Option<Vec<(u64, u64)>>,
  seed_to_soil: Vec<(u64, u64, u64)>,
  soil_to_fertilizer: Vec<(u64, u64, u64)>,
  fertilizer_to_water: Vec<(u64, u64, u64)>,
  water_to_light: Vec<(u64, u64, u64)>,
  light_to_temperature: Vec<(u64, u64, u64)>,
  temperature_to_humidity: Vec<(u64, u64, u64)>,
  humidity_to_location: Vec<(u64, u64, u64)>,
}

impl Almanach {
  pub fn new() -> Almanach {
    Almanach {
      seeds: None,
      seeds_range: None,
      seed_to_soil: Vec::new(),
      soil_to_fertilizer: Vec::new(),
      fertilizer_to_water: Vec::new(),
      water_to_light: Vec::new(),
      light_to_temperature: Vec::new(),
      temperature_to_humidity: Vec::new(),
      humidity_to_location: Vec::new(),
    }
  }

  pub fn find_in_map(&self, map: &Vec<(u64, u64, u64)>, value: u64) -> u64 {
    let slice = map.par_iter().find_first(|(_end, start, length)| {
      value >= *start && value < *start + *length
    });
    match slice {
      Some((end, start, length)) => {
        (*end + *length) - (*start + *length - value)
      }
      None => value
    }
  }

  pub fn find(&self, seed: u64) -> u64 {
    let soil = self.find_in_map(&self.seed_to_soil, seed);
    let fertilizer = self.find_in_map(&self.soil_to_fertilizer, soil);
    let water = self.find_in_map(&self.fertilizer_to_water, fertilizer);
    let light = self.find_in_map(&self.water_to_light, water);
    let temperature = self.find_in_map(&self.light_to_temperature, light);
    let humidity = self.find_in_map(&self.temperature_to_humidity, temperature);
    let location = self.find_in_map(&self.humidity_to_location, humidity);
    location
  }
}

fn build_seeds(lines: &mut Lines<'_>, almanach: &mut Almanach) {
  almanach.seeds = Some(Vec::new());
  let seeds = lines.next().unwrap().replace("seeds: ", "");
  for seed in seeds.split_whitespace() {
    let seed = seed.parse::<u64>().unwrap();
    almanach.seeds.as_mut().unwrap().push(seed);
  }
}

fn build_seeds_ranges(lines: &mut Lines<'_>, almanach: &mut Almanach) {
  let seeds_range = lines.next().unwrap().replace("seeds: ", "");
  almanach.seeds_range = Some(Vec::new());
  let mut start = None;
  for seed_range in seeds_range.split_whitespace() {
    let number = seed_range.parse::<u64>().unwrap();
    match start {
      Some(s) => {
        almanach.seeds_range.as_mut().unwrap().push((s, number));
        start = None;
      }
      None => {
        start = Some(number);
      }
    }
  }
}

fn build_ranges(lines: &mut Lines<'_>, almanach: &mut Almanach) {
  let mut current_map = 0;
  while let Some(line) = lines.next() {
    if line.trim().is_empty() {
      continue;
    }
    if line.starts_with("seed-to-soil") {
      current_map = 1;
    } else if line.starts_with("soil-to-fertilizer") {
      current_map = 2;
    } else if line.starts_with("fertilizer-to-water") {
      current_map = 3;
    } else if line.starts_with("water-to-light") {
      current_map = 4;
    } else if line.starts_with("light-to-temperature") {
      current_map = 5;
    } else if line.starts_with("temperature-to-humidity") {
      current_map = 6;
    } else if line.starts_with("humidity-to-location") {
      current_map = 7;
    } else {
      let parts = line.split(" ").collect::<Vec<&str>>();
      let destination_range_start = parts[0].parse::<u64>().unwrap();
      let source_range_start = parts[1].parse::<u64>().unwrap();
      let range_length = parts[2].parse::<u64>().unwrap();

      match current_map {
        1 => {
          almanach.seed_to_soil.push((destination_range_start, source_range_start, range_length));
        }
        2 => {
          almanach.soil_to_fertilizer.push((destination_range_start, source_range_start, range_length));
        }
        3 => {
          almanach.fertilizer_to_water.push((destination_range_start, source_range_start, range_length));
        }
        4 => {
          almanach.water_to_light.push((destination_range_start, source_range_start, range_length));
        }
        5 => {
          almanach.light_to_temperature.push((destination_range_start, source_range_start, range_length));
        }
        6 => {
          almanach.temperature_to_humidity.push((destination_range_start, source_range_start, range_length));
        }
        7 => {
          almanach.humidity_to_location.push((destination_range_start, source_range_start, range_length));
        }
        _ => panic!("Unknown map")
      }
    }
  }
}

pub fn part1(path: &str) -> u64 {
  let content = fs::read_to_string(path).expect("Unable to read file");

  let mut almanach = Almanach::new();
  let mut lines = content.lines();

  build_seeds(&mut lines, &mut almanach);

  build_ranges(&mut lines, &mut almanach);

  // Retrieve the lowest location
  let location = almanach.seeds.as_ref().unwrap().par_iter().map(
    |seed| {
      almanach.find(*seed)
    }
  ).collect::<Vec<u64>>();

  *location.par_iter().min().unwrap()
}

pub fn part2(path: &str) -> u64 {
  let content = fs::read_to_string(path).expect("Unable to read file");

  let mut almanach = Almanach::new();
  let mut lines = content.lines();

  build_seeds_ranges(&mut lines, &mut almanach);

  build_ranges(&mut lines, &mut almanach);

  // Retrieve the lowest location
  // This might take some time (17 minutes on my machine)
  almanach.seeds_range.as_ref().unwrap().iter().map(
    |(start, length)| {
      (*start..=(*start + length)).into_par_iter().map(
        |seed| {
          almanach.find(seed)
        }
      ).min().unwrap()
    }
  ).min().unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part1() {
    assert_eq!(part1("src/day5/test-part1.txt"), 35);
  }

  #[test]
  fn test_part2() {
    assert_eq!(part2("src/day5/test-part2.txt"), 46);
  }
}