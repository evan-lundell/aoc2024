use crate::days::Day;
use std::collections::{HashMap, HashSet};

use grid::Grid;

static OBSTACLE: char = '#';
static ROBOT: char = '^';

pub struct Day06;

impl Day for Day06 {
    fn part1(contents: &str) -> u64 {
        let grid = populate_grid(contents);
        let start_index = find_start(&grid).unwrap();
        let start_direction = Direction::Up;
        let mut visited: HashMap<(usize, usize), HashSet<Direction>> = HashMap::new();
        let spaces_visited = solve_grid(&grid, &mut visited, start_index, start_direction);
        spaces_visited as u64
    }

    fn part2(contents: &str) -> u64 {
        let grid = populate_grid(contents);
        let start_index = find_start(&grid).unwrap();
        let start_direction = Direction::Up;
        let mut visited: HashMap<(usize, usize), HashSet<Direction>> = HashMap::new();
        solve_grid(&grid, &mut visited, start_index, start_direction);
        
        let mut obstacles_added: u64 = 0;
        for (index, _) in visited.iter() {
            let mut new_grid = grid.clone();
            new_grid[*index] = OBSTACLE;
            let mut new_visited: HashMap<(usize, usize), HashSet<Direction>> = HashMap::new();
            if solve_grid(&new_grid, &mut new_visited, start_index, start_direction) == -1 {
                obstacles_added += 1;
            }
        }

        obstacles_added
    }
}
fn populate_grid(contents: &str) -> Grid<char> {
    let rows: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    Grid::from_vec(rows.concat(), rows[0].len())
}

fn get_next_index_and_direction(grid: &Grid<char>, current_index: &(usize, usize), current_direction: &Direction) -> Option<((usize, usize), Direction)> {
    let mut next_direction = *current_direction;
    let next_index = match next_direction {
        Direction::Up => (current_index.0 as isize - 1, current_index.1 as isize),
        Direction::Down => (current_index.0 as isize + 1, current_index.1 as isize),
        Direction::Left => (current_index.0 as isize, current_index.1 as isize - 1),
        Direction::Right => (current_index.0 as isize, current_index.1 as isize + 1),
    };
    
    if next_index.0 < 0
        || next_index.1 < 0
        || next_index.0 as usize >= grid.rows()
        || next_index.1 as usize >= grid.cols()
    {
        return None;
    }
    
    let mut next_index = (next_index.0 as usize, next_index.1 as usize);
    while grid[next_index] == OBSTACLE {
        match next_direction {
            Direction::Up => {
                next_direction = Direction::Right;
                next_index = (current_index.0, current_index.1 + 1);
            }
            Direction::Down => {
                next_direction = Direction::Left;
                next_index = (current_index.0, current_index.1 - 1);
            }
            Direction::Left => {
                next_direction = Direction::Up;
                next_index = (current_index.0 - 1, current_index.1);
            }
            Direction::Right => {
                next_direction = Direction::Down;
                next_index = (current_index.0 + 1, current_index.1);
            }
        }
    }
    Some((next_index, next_direction))
}

fn find_start(grid: &Grid<char>) -> Option<(usize, usize)> {
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            if grid[(row, col)] == ROBOT {
                return Some((row, col));
            }
        }
    }
    None
}

fn solve_grid(grid: &Grid<char>, visited: &mut HashMap<(usize, usize), HashSet<Direction>>, start_index: (usize, usize), start_direction: Direction) -> i32 {
    let mut current_index = start_index;
    let mut current_direction = start_direction;

    loop {
        visited.entry(current_index)
            .or_insert(HashSet::new())
            .insert(current_direction);
        let (next_index, next_direction) = match get_next_index_and_direction(&grid, &current_index, &current_direction) {
            Some((next_index, next_direction)) => (next_index, next_direction),
            None => break,
        };

        // check for loop
        if visited.contains_key(&next_index) {
            let directions = visited.get(&next_index).unwrap();
            if directions.contains(&next_direction) {
                return -1;
            }
        }

        current_index = next_index;
        current_direction = next_direction;
    }

    visited.len() as i32
}


#[derive(Hash, Eq, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(Day06::part1(contents), 41);
    }

    #[test]
    fn test_part2() {
        let contents = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(Day06::part2(contents), 6);
    }
}
