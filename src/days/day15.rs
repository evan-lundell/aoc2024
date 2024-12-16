use std::collections::HashMap;

use crate::days::Day;

pub struct Day15;

impl Day for Day15 {
    fn part1(contents: &str) -> u64 {
        solve(contents, false)
    }

    fn part2(contents: &str) -> u64 {
        solve(contents, true)
    }
}

fn solve(contents: &str, expand_grid: bool) -> u64 {
    let (mut grid, moves) = parse_input(contents, expand_grid);
        let mut position = find_starting_position(&grid);
        for m in moves.iter() {
            let direction: (isize, isize) = match m {
                'v' => (1, 0),
                '^' => (-1, 0),
                '<' => (0, -1),
                '>' => (0, 1),
                _ => {
                    continue;
                }
            };
            let mut move_set: HashMap<(usize, usize), char> = HashMap::new();
            move_set.insert(position, '.');
            if can_move(&grid, position, direction, &mut move_set, false) {
                for (pos, c) in move_set.iter() {
                    grid[pos.0][pos.1] = *c;
                }
                position = ((position.0 as isize + direction.0) as usize, (position.1 as isize + direction.1) as usize);
            }
        }

        calculate_obstacle_value(&grid)
}

fn parse_input(contents: &str, expand: bool) -> (Vec<Vec<char>>, Vec<char>) {
    let parts: Vec<&str> = contents.split("\n\n").collect();
    
    let grid: Vec<Vec<char>> = parts[0].lines().map(|line| {
        line.chars().flat_map(|c| {
            match (expand, c) {
                (true, '#') => "##".chars().collect::<Vec<char>>(),
                (true, '.') => "..".chars().collect::<Vec<char>>(),
                (true, 'O') => "[]".chars().collect::<Vec<char>>(),
                (true, '@') => "@.".chars().collect::<Vec<char>>(),
                (_, _) => vec![c],
            }
        }).collect()
    }).collect();

    let moves: Vec<char> = parts[1].chars().collect();

    (grid, moves)
}

fn find_starting_position(grid: &Vec<Vec<char>>) -> (usize, usize)  {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '@' {
                return (i, j);
            }
        }
    }

    (0, 0)
}

fn can_move(grid: &Vec<Vec<char>>, current_position: (usize, usize), direction: (isize, isize), move_set: &mut HashMap<(usize, usize), char>, part_of_pair: bool) -> bool {
    let next_position = ((current_position.0 as isize + direction.0) as usize, (current_position.1 as isize + direction.1) as usize);
    let next_cell = grid[next_position.0][next_position.1];
    let current_cell = grid[current_position.0][current_position.1];
    if next_cell == '#' {
        return false;
    }

    if next_cell != '.' {
        if !can_move(grid, next_position, direction, move_set, false) {
            return false;
        }
    }

    if direction == (1, 0) || direction == (-1, 0) {
        if current_cell == '[' && !part_of_pair {
            if !can_move(grid, (current_position.0, current_position.1 + 1), direction, move_set, true) {
                return false;
            }
        } else if current_cell == ']' && !part_of_pair {
            if !can_move(grid, (current_position.0, current_position.1 - 1), direction, move_set, true) {
                return false;
            }
        } else if part_of_pair {
            move_set.entry(current_position).or_insert('.');
        }
    }

    move_set.insert(next_position, current_cell);

    true
}

fn calculate_obstacle_value(grid: &Vec<Vec<char>>) -> u64 {
    let mut total_value: u64 = 0;
    for (row, line) in grid.iter().enumerate() {
        for (col, &cell) in line.iter().enumerate() {
            if cell == 'O' || cell == '[' {
                total_value += 100 * row as u64 + col as u64;
            }
        }
    }
    total_value
}

fn _print_grid(grid: &Vec<Vec<char>>) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            print!("{}", grid[i][j]);
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        assert_eq!(Day15::part1(input), 10092);
    }

    #[test]
    fn test_part2() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        assert_eq!(Day15::part2(input), 9021);
    }
}
