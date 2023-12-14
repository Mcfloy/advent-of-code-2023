use log::info;

mod day3;
mod day4;

fn main() {
  dotenv::dotenv().ok();
  env_logger::init();
  info!("Day 3 part 1: {}", day3::part1("src/day3/input.txt"));
  info!("Day 3 part 2: {}", day3::part2("src/day3/input.txt"));

  info!("Day 4 part 1: {}", day4::part1("src/day4/input.txt"));
  info!("Day 4 part 2: {}", day4::part2("src/day4/input.txt"));
}
