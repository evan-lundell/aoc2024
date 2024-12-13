use std::collections::HashSet;

use crate::days::Day;

pub struct Day12;

#[derive(Debug)]
struct Plot {
    area: u32,
    perimeter: u32,
    corners: u32
}

impl Day for Day12 {
    fn part1(contents: &str) -> u64 {
        let mut plots: Vec<Plot> = Vec::new();
        let grid = parse_input(contents);
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if visited.contains(&(i, j)) {
                    continue;
                }
                let (area, perimeter, corners) = find_plot(&grid, &mut visited, (i, j), grid[i][j]);
                plots.push(Plot { area, perimeter, corners });
            }
        }

        plots.iter().map(|p| (p.area * p.perimeter) as u64).sum()
    }

    fn part2(contents: &str) -> u64 {
        let mut plots: Vec<Plot> = Vec::new();
        let grid = parse_input(contents);
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if visited.contains(&(i, j)) {
                    continue;
                }
                let (area, perimeter, corners) = find_plot(&grid, &mut visited, (i, j), grid[i][j]);
                plots.push(Plot { area, perimeter, corners });
            }
        }

        plots.iter().map(|p| (p.area * p.corners) as u64).sum()
    }
}

fn parse_input(contents: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in contents.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }
    grid
}

fn find_plot(grid: &Vec<Vec<char>>, visited: &mut HashSet<(usize, usize)>, current_index: (usize, usize), current_plant: char) -> (u32, u32, u32) {
    let mut area = 1;
    let mut perimeter = 0;

    if current_index.0 == 0 || (current_index.0 > 0 && grid[current_index.0 - 1][current_index.1] != current_plant) {
        perimeter += 1;
    }
    if current_index.0 == grid.len() - 1 || (current_index.0 < grid.len() - 1 && grid[current_index.0 + 1][current_index.1] != current_plant) {
        perimeter += 1;
    }
    if current_index.1 == 0 || (current_index.1 > 0 && grid[current_index.0][current_index.1 - 1] != current_plant) {
        perimeter += 1;
    }
    if current_index.1 == grid[current_index.0].len() - 1 || (current_index.1 < grid[current_index.0].len() - 1 && grid[current_index.0][current_index.1 + 1] != current_plant) {
        perimeter += 1;
    }
    visited.insert(current_index);

    let mut corners = num_of_corners(grid, current_index, current_plant);

    if current_index.0 > 0 && grid[current_index.0 - 1][current_index.1] == current_plant && !visited.contains(&(current_index.0 - 1, current_index.1)) {
        let (a, p, c) = find_plot(grid, visited, (current_index.0 - 1, current_index.1), current_plant);
        area += a;
        perimeter += p;
        corners += c;
    }
    if current_index.0 < grid.len() - 1 && grid[current_index.0 + 1][current_index.1] == current_plant && !visited.contains(&(current_index.0 + 1, current_index.1)) {
        let (a, p, c) = find_plot(grid, visited, (current_index.0 + 1, current_index.1), current_plant);
        area += a;
        perimeter += p;
        corners += c;
    }
    if current_index.1 > 0 && grid[current_index.0][current_index.1 - 1] == current_plant && !visited.contains(&(current_index.0, current_index.1 - 1)) {
        let (a, p, c) = find_plot(grid, visited, (current_index.0, current_index.1 - 1), current_plant);
        area += a;
        perimeter += p;
        corners += c;
    }
    if current_index.1 < grid[current_index.0].len() - 1 && grid[current_index.0][current_index.1 + 1] == current_plant && !visited.contains(&(current_index.0, current_index.1 + 1)) {
        let (a, p, c) = find_plot(grid, visited, (current_index.0, current_index.1 + 1), current_plant);
        area += a;
        perimeter += p;
        corners += c;
    }
    (area, perimeter, corners)
}

fn num_of_corners (grid: &Vec<Vec<char>>, current_position: (usize, usize), current_plant: char) -> u32 {
    // find convex corners
    let mut corners = 0;
    // mask for the 4 directions: up, right, down, left
    let mut mask = 0b0000;
    if current_position.0 == 0 || grid[current_position.0 - 1][current_position.1] != current_plant {
        mask |= 0b1000;
    }
    if current_position.1 == grid[current_position.0].len() - 1 || grid[current_position.0][current_position.1 + 1] != current_plant {
        mask |= 0b0100;
    }
    if current_position.0 == grid.len() - 1 || grid[current_position.0 + 1][current_position.1] != current_plant {
        mask |= 0b0010;
    }
    if current_position.1 == 0 || grid[current_position.0][current_position.1 - 1] != current_plant {
        mask |= 0b0001;
    }

    corners += match mask {
        0b1111 => 4,
        0b1110 | 0b1011 | 0b0111 | 0b1101 => 2,
        0b1100 | 0b1001 | 0b0011 | 0b0110 => 1,
        _ => 0
    };

    // find diagonal corners

    // top-left
    if current_position.0 > 0 && current_position.1 > 0 && grid[current_position.0 - 1][current_position.1 - 1] != current_plant && grid[current_position.0 - 1][current_position.1] == current_plant && grid[current_position.0][current_position.1 - 1] == current_plant {
        corners += 1;
    }

    // top-right
    if current_position.0 > 0 && current_position.1 < grid[0].len() - 1 && grid[current_position.0 - 1][current_position.1 + 1] != current_plant && grid[current_position.0 - 1][current_position.1] == current_plant && grid[current_position.0][current_position.1 + 1] == current_plant {
        corners += 1;
    }

    // bottom-left
    if current_position.0 < grid.len() - 1 && current_position.1 > 0 && grid[current_position.0 + 1][current_position.1 - 1] != current_plant && grid[current_position.0 + 1][current_position.1] == current_plant && grid[current_position.0][current_position.1 - 1] == current_plant {
        corners += 1;
    }

    // bottom-right
    if current_position.0 < grid.len() - 1 && current_position.1 < grid[0].len() - 1 && grid[current_position.0 + 1][current_position.1 + 1] != current_plant && grid[current_position.0 + 1][current_position.1] == current_plant && grid[current_position.0][current_position.1 + 1] == current_plant {
        corners += 1;
    }

    corners

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!(Day12::part1(input), 1930);
    }

    #[test]
    fn test_part2() {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!(Day12::part2(input), 1206);
    }
}
