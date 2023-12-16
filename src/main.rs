use log::info;

mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
  dotenv::dotenv().ok();
  env_logger::init();
  // info!("Day 3 part 1: {}", day3::part1("src/day3/input.txt"));
  // info!("Day 3 part 2: {}", day3::part2("src/day3/input.txt"));
  //
  // info!("Day 4 part 1: {}", day4::part1("src/day4/input.txt"));
  // info!("Day 4 part 2: {}", day4::part2("src/day4/input.txt"));

  // info!("Day 5 part 1 begin.");
  // info!("Day 5 part 1: {}", day5::part1("src/day5/input.txt"));
  //
  // info!("Day 5 part 2 begin.");
  // info!("Day 5 part 2: {}", day5::part2("src/day5/input.txt"));

  info!("Day 6 part 1 begin.");
  info!("Day 6 part 1: {}", day6::part1("src/day6/input.txt"));

  info!("Day 6 part 2 begin.");
  info!("Day 6 part 2: {}", day6::part2("src/day6/input.txt"));
}
