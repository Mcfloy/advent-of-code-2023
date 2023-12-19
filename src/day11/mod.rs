use std::collections::HashMap;
use std::fs;

pub fn day11(path: &str, distance_per_empty: u64) -> u64 {
    let content = fs::read_to_string(path).expect("Failed to read file");

    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in content.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }

    let mut galaxies: Vec<(u64, u64)> = Vec::new();
    let mut simulated_y = 0;
    for y in 0..grid.len() {
        let mut simulated_x = 0;
        let is_row_empty = grid[y].iter().all(|&c| c == '.');
        if is_row_empty {
            simulated_y += distance_per_empty;
        } else {
            simulated_y += 1;
        }
        for x in 0..grid[y].len() {
            let is_column_empty = grid.iter().all(|row| row[x] == '.');
            if is_column_empty {
                simulated_x += distance_per_empty;
            } else {
                simulated_x += 1;
            }
            if grid[y][x] == '#' {
                galaxies.push((simulated_y, simulated_x));
            }
        }
    }

    let mut shortest_path: HashMap<(u64, u64, u64, u64), u64> = HashMap::new();
    let mut sum = 0;
    for (y, x) in &galaxies {
        for (y2, x2) in &galaxies {
            if y == y2 && x == x2 {
                continue;
            }
            let distance: u64 = {
                let y = *y as i64;
                let x = *x as i64;
                let y2 = *y2 as i64;
                let x2 = *x2 as i64;
                (y2 - y).abs() as u64 + (x2 - x).abs() as u64
            };
            if shortest_path.contains_key(&(*y2, *x2, *y, *x)) || shortest_path.contains_key(&(*y, *x, *y2, *x2)) {
                continue;
            }
            shortest_path.insert((*y, *x, *y2, *x2), distance);
            sum += distance;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(day11("src/day11/test1.txt", 2), 374);
    }

    #[test]
    fn test_part2() {
        assert_eq!(day11("src/day11/test1.txt", 10), 1030);
    }

    #[test]
    fn test_part3() {
        assert_eq!(day11("src/day11/test1.txt", 100), 8410);
    }
}