use crate::days::Day;
use std::collections::HashSet;

const UP: (isize, isize) = (-1, 0);
const DOWN: (isize, isize) = (1, 0);
const LEFT: (isize, isize) = (0, -1);
const RIGHT: (isize, isize) = (0, 1);

pub struct Day10;

impl Day for Day10 {



    fn part1(contents: &str) -> u64 {
        let mut grid: Vec<Vec<u32>> = Vec::new();
        let trailheads = populate_grid_and_get_trailheads(contents, &mut grid);
        let mut total_trail_score: u64 = 0;
        
        for trailhead in trailheads {
            let mut trailends: HashSet<(usize, usize)> = HashSet::new();
            _ = find_trail_score(&grid, &mut trailends, trailhead, (0, 0));
            total_trail_score += trailends.len() as u64;
        }

        total_trail_score
    }

    fn part2(contents: &str) -> u64 {
        let mut grid: Vec<Vec<u32>> = Vec::new();
        let trailheads = populate_grid_and_get_trailheads(contents, &mut grid);
        let mut ratings: u64 = 0;
        
        for trailhead in trailheads {
            let mut trailends: HashSet<(usize, usize)> = HashSet::new();
            ratings += find_trail_score(&grid, &mut trailends, trailhead, (0, 0));
        }

        ratings
    }

}
fn find_trail_score(grid: &Vec<Vec<u32>>, trailends: &mut HashSet<(usize, usize)>, coords: (usize, usize), from_direction: (isize, isize)) -> u64 {
    let current_value = grid[coords.0][coords.1];

    if grid[coords.0][coords.1] == 9 {
        trailends.insert(coords);
        return 1;
    }

    let mut trail_score: u64 = 0;
    if from_direction != UP && coords.0 > 0 && grid[coords.0 - 1][coords.1] == current_value + 1 {
        trail_score += find_trail_score(grid, trailends, (coords.0 - 1, coords.1), DOWN);
    }
    if from_direction != DOWN && coords.0 < grid.len() - 1 && grid[coords.0 + 1][coords.1] == current_value + 1 {
        trail_score += find_trail_score(grid, trailends, (coords.0 + 1, coords.1), UP);
    }
    if from_direction != LEFT && coords.1 > 0 && grid[coords.0][coords.1 - 1] == current_value + 1 {
        trail_score += find_trail_score(grid, trailends, (coords.0, coords.1 - 1), RIGHT);
        
    }
    if from_direction != RIGHT && coords.1 < grid[0].len() - 1 && grid[coords.0][coords.1 + 1] == current_value + 1 {
        trail_score += find_trail_score(grid, trailends, (coords.0, coords.1 + 1), LEFT);
    }

    trail_score
}

fn populate_grid_and_get_trailheads(contents: &str, grid: &mut Vec<Vec<u32>>) -> HashSet<(usize, usize)> {
    let mut trailheads: HashSet<(usize, usize)> = HashSet::new();
    for (row_index, row) in contents.lines().enumerate() {
        let mut grid_row: Vec<u32> = Vec::new();
        for (col_index, c) in row.chars().map(|c| { c.to_digit(10).unwrap() }).enumerate() {
            grid_row.push(c);
            if c == 0 {
                trailheads.insert((row_index, col_index));
            }
        }
        grid.push(grid_row);
    }

    trailheads
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!(Day10::part1(contents), 36);
    }

    #[test]
    fn test_part2() {
        let contents = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!(Day10::part2(contents), 81);
    }
}
