use std::collections::HashMap;
use std::fs;
use log::debug;

fn next_cells(grid: &Vec<Vec<char>>, y: usize, x: usize) -> Vec<(usize, usize)> {
    let mut cells: Vec<(usize, usize)> = vec![];
    // Inner ny and nx variables are referring to the next cell to add.
    match grid[y][x] {
        'S' => {
            if let Some((ny, nx)) = check_left_cell(&grid, y, x) {
                cells.push((ny, nx));
            }
            if let Some((ny, nx)) = check_right_cell(&grid, y, x) {
                cells.push((ny, nx));
            }
            if let Some((ny, nx)) = check_top_cell(&grid, y, x) {
                cells.push((ny, nx));
            }
            if let Some((ny, nx)) = check_bottom_cell(&grid, y, x) {
                cells.push((ny, nx));
            }
        }
        '-' => {
            if let Some((ny, nx)) = check_left_cell(&grid, y, x) {
                cells.push((ny, nx));
            }
            if let Some((ny, nx)) = check_right_cell(&grid, y, x) {
                cells.push((ny, nx));
            }
        }
        '|' => {
            if let Some((ny, nx)) = check_top_cell(&grid, y, x) {
                cells.push((ny, nx));
            }
            if let Some((ny, nx)) = check_bottom_cell(&grid, y, x) {
                cells.push((ny, nx));
            }
        }
        'F' => {
            if let Some((ny, nx)) = check_right_cell(&grid, y, x) {
                cells.push((ny, nx));
            }
            if let Some((ny, nx)) = check_bottom_cell(&grid, y, x) {
                cells.push((ny, nx));
            }
        }
        '7' => {
            if let Some((ny, nx)) = check_left_cell(&grid, y, x) {
                cells.push((ny, nx));
            }
            if let Some((ny, nx)) = check_bottom_cell(&grid, y, x) {
                cells.push((ny, nx));
            }
        }
        'J' => {
            if let Some((ny, nx)) = check_left_cell(&grid, y, x) {
                cells.push((ny, nx));
            }
            if let Some((ny, nx)) = check_top_cell(&grid, y, x) {
                cells.push((ny, nx));
            }
        }
        'L' => {
            if let Some((ny, nx)) = check_right_cell(&grid, y, x) {
                cells.push((ny, nx));
            }
            if let Some((ny, nx)) = check_top_cell(&grid, y, x) {
                cells.push((ny, nx));
            }
        }
        _ => {}
    }
    cells
}

fn check_left_cell(grid: &Vec<Vec<char>>, y: usize, x: usize) -> Option<(usize, usize)> {
    if x == 0 {
        return None;
    }
    let left_cell = grid[y][x - 1];
    match left_cell {
        '-' | 'L' | 'F' => Some((y, x - 1)),
        _ => None
    }
}

fn check_right_cell(grid: &Vec<Vec<char>>, y: usize, x: usize) -> Option<(usize, usize)> {
    if x == grid[y].len() - 1 {
        return None;
    }
    let right_cell = grid[y][x + 1];
    match right_cell {
        '-' | 'J' | '7' => Some((y, x + 1)),
        _ => None
    }
}

fn check_top_cell(grid: &Vec<Vec<char>>, y: usize, x: usize) -> Option<(usize, usize)> {
    if y == 0 {
        return None;
    }
    let top_cell = grid[y - 1][x];
    match top_cell {
        '|' | 'F' | '7' => Some((y - 1, x)),
        _ => None
    }
}

fn check_bottom_cell(grid: &Vec<Vec<char>>, y: usize, x: usize) -> Option<(usize, usize)> {
    if y == grid.len() - 1 {
        return None;
    }
    let bottom_cell = grid[y + 1][x];
    match bottom_cell {
        '|' | 'J' | 'L' => Some((y + 1, x)),
        _ => None
    }
}

pub fn part1(path: &str) -> u64 {
    let content = fs::read_to_string(path).expect("Failed to read file");

    let mut grid: Vec<Vec<char>> = vec![];
    let mut cells_with_distance: HashMap<(usize, usize), u64> = HashMap::new();
    let mut start = (0, 0, 0);
    for (y, line) in content.lines().enumerate() {
        let mut row: Vec<char> = vec![];
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (y, x, 0);
            }
            row.push(c);
        }
        grid.push(row);
    }

    let mut cells_to_explore: Vec<(usize, usize, u64)> = vec![start];
    while cells_to_explore.len() > 0 {
        let (y, x, distance) = cells_to_explore[0];
        cells_to_explore.remove(0);
        cells_with_distance.insert((y, x), distance);
        let next_cells = next_cells(&grid, y, x);
        for (ny, nx) in next_cells {
            if !cells_with_distance.contains_key(&(ny, nx)) {
                cells_to_explore.push((ny, nx, distance + 1));
            }
        }
    }

    // if log_enabled!(log::Level::Debug) {
    //   for (y, row) in grid.iter().enumerate() {
    //     for (x, c) in row.iter().enumerate() {
    //       if cells_with_distance.contains_key(&(y, x)) {
    //         print!("{} ", cells_with_distance.get(&(y, x)).unwrap());
    //       } else {
    //         print!("{} ", c);
    //       }
    //     }
    //     println!();
    //   }
    // }

    cells_with_distance.values().max().unwrap().clone()
}

pub fn part2(path: &str) -> u64 {
    let content = fs::read_to_string(path).expect("Failed to read file");

    let mut grid: Vec<Vec<char>> = vec![];
    let mut cells_with_distance: HashMap<(usize, usize), u64> = HashMap::new();
    let mut start = (0, 0, 0);
    for (y, line) in content.lines().enumerate() {
        let mut row: Vec<char> = vec![];
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (y, x, 0);
            }
            row.push(c);
        }
        grid.push(row);
    }

    let mut cells_to_explore: Vec<(usize, usize, u64)> = vec![start];
    while cells_to_explore.len() > 0 {
        let (y, x, distance) = cells_to_explore[0];
        cells_to_explore.remove(0);
        cells_with_distance.insert((y, x), distance);
        let next_cells = next_cells(&grid, y, x);
        for (ny, nx) in next_cells {
            if !cells_with_distance.contains_key(&(ny, nx)) {
                cells_to_explore.push((ny, nx, distance + 1));
            }
        }
    }

    let mut new_grid: Vec<Vec<char>> = vec![];
    for (y, row) in grid.iter().enumerate() {
        let mut new_row: Vec<char> = vec![];
        for (x, c) in row.iter().enumerate() {
            if cells_with_distance.contains_key(&(y, x)) {
                if *c == 'S' {
                    // Change depending on the input you've got.
                    new_row.push('L');
                } else {
                    new_row.push(*c);
                }
            } else {
                new_row.push('.');
            }
        }
        new_grid.push(new_row);
    }

    let mut counter = 0;
    for y in 0..new_grid.len() {
        let mut inside_loop = false;
        for x in 0..new_grid[y].len() {
            match new_grid[y][x] {
                '|' | 'J' | 'L' => {
                    inside_loop = !inside_loop;
                    debug!("{}", new_grid[y][x]);
                }
                '.' => {
                    if inside_loop {
                        counter += 1;
                        debug!("I");
                    } else {
                        debug!("O");
                    }
                }
                _ => {
                    debug!("{}", new_grid[y][x]);
                }
            }
        }
        debug!("\n");
    }

    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("src/day10/test-part1.txt"), 4);
    }

    #[test]
    fn test_part1b() {
        assert_eq!(part1("src/day10/test-part1b.txt"), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("src/day10/test-part2.txt"), 4);
    }

    #[test]
    fn test_part2b() {
        assert_eq!(part2("src/day10/test-part2b.txt"), 8);
    }

    #[test]
    fn test_part2c() {
        assert_eq!(part2("src/day10/test-part2c.txt"), 10);
    }
}

