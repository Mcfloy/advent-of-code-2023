use log::info;

mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;

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

    // info!("Day 6 part 1 begin.");
    // info!("Day 6 part 1: {}", day6::part1("src/day6/input.txt"));
    //
    // info!("Day 6 part 2 begin.");
    // info!("Day 6 part 2: {}", day6::part2("src/day6/input.txt"));
    //
    // info!("Day 7 part 1 begin.");
    // info!("Day 7 part 1: {}", day7::part1("src/day7/input.txt"));
    //
    // info!("Day 7 part 2 begin.");
    // info!("Day 7 part 2: {}", day7::part2("src/day7/input.txt"));
    //
    // info!("Day 8 part 1 begin.");
    // info!("Day 8 part 1: {}", day8::part1("src/day8/input.txt"));
    //
    // info!("Day 8 part 2 begin.");
    // info!("Day 8 part 2: {}", day8::part2("src/day8/input.txt"));
    //
    // info!("Day 9 part 1 begin.");
    // info!("Day 9 part 1: {}", day9::part1("src/day9/input.txt"));
    //
    // info!("Day 9 part 2 begin.");
    // info!("Day 9 part 2: {}", day9::part2("src/day9/input.txt"));
    //
    // info!("Day 10 part 1 begin.");
    // info!("Day 10 part 1: {}", day10::part1("src/day10/input.txt"));
    //
    // info!("Day 10 part 2 begin.");
    // info!("Day 10 part 2: {}", day10::part2("src/day10/input.txt"));
    //
    // info!("Day 11 part 1 begin.");
    // info!("Day 11 part 1: {}", day11::day11("src/day11/input.txt", 2));
    //
    // info!("Day 11 part 2 begin.");
    // info!("Day 11 part 2: {}", day11::day11("src/day11/input.txt", 1000000));

    info!("Day 12 part 1 begin.");
    info!("Day 12 part 1: {}", day12::part1("src/day12/input.txt"));

    info!("Day 12 part 2 begin.");
    info!("Day 12 part 2: {}", day12::part2("src/day12/input.txt"));
}
