use std::collections::HashMap;
use std::fs;
use log::debug;

fn is_a_special_character(c: char) -> bool {
    match c {
        '.' => false,
        '0'..='9' => false,
        _ => true
    }
}

fn generate_grid(path: &str) -> Vec<Vec<char>> {
    let file = fs::read_to_string(path).expect("Failed to read file");

    let content = file.lines().collect::<Vec<&str>>();

    // Convert into a 2D array
    content.iter()
        .map(|line| {
            let mut line = line.chars().collect::<Vec<char>>();
            line.push('.');
            line
        })
        .collect::<Vec<Vec<char>>>()
}

pub fn part1(path: &str) -> i32 {
    // Convert into a 2D array
    let grid = generate_grid(path);

    let mut result = 0;

    for y in 0..grid.len() {
        let mut x = 0;
        while x < grid[y].len() {
            if grid[y][x].is_numeric() {
                let mut add_number = false;
                // Retrieve the current number we are on
                let mut number = String::from(grid[y][x]);
                let begin_x = if x == 0 { 0 } else { x - 1 };
                let mut end_x = if x == grid[y].len() - 1 { x } else { x + 1 };
                while end_x < grid[y].len() {
                    if !grid[y][end_x].is_numeric() {
                        break;
                    }
                    number = format!("{}{}", number, grid[y][end_x]);
                    end_x += 1;
                }
                // Check around the current number if there is a special symbol
                if y != 0 {
                    // Check above
                    let extreme_end_x = if end_x >= grid[y].len() { end_x } else { end_x + 1 };
                    for i in begin_x..extreme_end_x {
                        if is_a_special_character(grid[y - 1][i]) {
                            debug!("Found a special character above");
                            add_number = true;
                            break;
                        }
                    }
                }
                if !add_number && x != 0 {
                    // Check the first position on the left
                    if is_a_special_character(grid[y][x - 1]) {
                        debug!("Found a special character on the left");
                        add_number = true;
                    }
                }
                if !add_number && end_x != grid[y].len() {
                    // Check the first position on the right
                    if is_a_special_character(grid[y][end_x]) {
                        debug!("Found a special character on the right");
                        add_number = true;
                    }
                }
                if !add_number && y != grid.len() - 1 {
                    let extreme_end_x = if end_x >= grid[y].len() { end_x } else { end_x + 1 };
                    // Check below
                    for i in begin_x..extreme_end_x {
                        if is_a_special_character(grid[y + 1][i]) {
                            debug!("Found a special character below");
                            add_number = true;
                            break;
                        }
                    }
                }
                debug!("{} {} {} {}", begin_x, end_x, number, add_number);
                if add_number {
                    result += number.parse::<i32>().unwrap();
                }
                x = end_x;
            } else {
                x += 1;
            }
        }
    }
    debug!("Day 3 Part 1: {}", result);
    result
}

pub fn part2(path: &str) -> i32 {
    // Convert into a 2D array
    let grid = generate_grid(path);

    let mut gears: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    let mut result = 0;

    for y in 0..grid.len() {
        let mut x = 0;
        while x < grid[y].len() {
            if grid[y][x].is_numeric() {
                let mut gear_position = (y, x);
                let mut add_number = false;
                // Retrieve the current number we are on
                let mut number = String::from(grid[y][x]);
                let begin_x = if x == 0 { 0 } else { x - 1 };
                let mut end_x = if x == grid[y].len() - 1 { x } else { x + 1 };
                while end_x < grid[y].len() {
                    if !grid[y][end_x].is_numeric() {
                        break;
                    }
                    number = format!("{}{}", number, grid[y][end_x]);
                    end_x += 1;
                }
                // Check around the current number if there is a special symbol
                if y != 0 {
                    // Check above
                    let extreme_end_x = if end_x >= grid[y].len() { end_x } else { end_x + 1 };
                    for i in begin_x..extreme_end_x {
                        if is_a_special_character(grid[y - 1][i]) {
                            debug!("Found a special character above");
                            add_number = true;
                            gear_position = (y - 1, i);
                            break;
                        }
                    }
                }
                if !add_number && x != 0 {
                    // Check the first position on the left
                    if is_a_special_character(grid[y][x - 1]) {
                        debug!("Found a special character on the left");
                        add_number = true;
                        gear_position = (y, x - 1);
                    }
                }
                if !add_number && end_x != grid[y].len() {
                    // Check the first position on the right
                    if is_a_special_character(grid[y][end_x]) {
                        debug!("Found a special character on the right");
                        add_number = true;
                        gear_position = (y, end_x);
                    }
                }
                if !add_number && y != grid.len() - 1 {
                    let extreme_end_x = if end_x >= grid[y].len() { end_x } else { end_x + 1 };
                    // Check below
                    for i in begin_x..extreme_end_x {
                        if is_a_special_character(grid[y + 1][i]) {
                            debug!("Found a special character below");
                            add_number = true;
                            gear_position = (y + 1, i);
                            break;
                        }
                    }
                }
                debug!("{} {} {} {}", begin_x, end_x, number, add_number);
                if add_number {
                    let gear = gears.entry(gear_position).or_insert(Vec::new());
                    gear.push(number.parse::<i32>().unwrap());
                }
                x = end_x;
            } else {
                x += 1;
            }
        }
    }

    for value in gears.values() {
        if value.len() == 2 {
            result += value[0] * value[1];
        }
    }

    debug!("Day 3 Part 2: {}", result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
       assert_eq!(part1("src/day3/test-part1.txt"), 4361);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("src/day3/test-part2.txt"), 467835);
    }
}