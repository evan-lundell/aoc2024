use crate::days::Day;

pub struct Day15;

impl Day for Day15 {
    fn part1(contents: &str) -> u64 {
        let (mut grid, moves) = parse_input(contents, false);
        let (mut y, mut x) = find_starting_position(&grid);
        for m in moves.iter() {
            (y, x) = match m {
                'v' => move_object(&mut grid, (y, x), (1, 0), '@'),
                '^' => move_object(&mut grid, (y, x), (-1, 0), '@'),
                '<' => move_object(&mut grid, (y, x), (0, -1), '@'),
                '>' => move_object(&mut grid, (y, x), (0, 1), '@'),
                _ => {
                    continue;
                }
            };
        }

        calculate_obstacle_value(&grid)
    }

    fn part2(_contents: &str) -> u64 {
        0
    }
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

fn move_object(grid: &mut Vec<Vec<char>>, position: (usize, usize), direction: (isize, isize), object: char) -> (usize, usize) {
    let next_position = ((position.0 as isize + direction.0) as usize, (position.1 as isize + direction.1) as usize);
    if grid[next_position.0][next_position.1] == '#' {
        return position;
    }

    if grid[next_position.0][next_position.1] == '.' {
        grid[next_position.0][next_position.1] = object;
        grid[position.0][position.1] = if object == '@' { '.' } else { 'O' };
        return next_position;
    }

    if grid[next_position.0][next_position.1] == 'O' {
        let new_object_position = move_object(grid, next_position, direction, 'O');
        if new_object_position != next_position {
            grid[next_position.0][next_position.1] = object;
            grid[position.0][position.1] = if object == '@' { '.' } else { 'O' };
            return next_position;
        }
    }

    position
}

fn calculate_obstacle_value(grid: &Vec<Vec<char>>) -> u64 {
    let mut total_value: u64 = 0;
    for (row, line) in grid.iter().enumerate() {
        for (col, &cell) in line.iter().enumerate() {
            if cell == 'O' {
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
        let input = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";
        assert_eq!(Day15::part2(input), 9021);
    }
}
