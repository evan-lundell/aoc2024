use crate::days::Day;

pub struct Day04;
impl Day for Day04 {

    fn part1(contents: &str) -> u64 {
        let lines: Vec<&str> = contents.lines().collect();
        let mut count = 0;
        for (line_index, line) in lines.iter().enumerate() {
            for (char_index, c) in line.chars().enumerate() {
                if c == 'X' {
                    // forward
                    if line.len() - char_index >= 4
                        && line.chars().nth(char_index + 1).unwrap() == 'M'
                        && line.chars().nth(char_index + 2).unwrap() == 'A'
                        && line.chars().nth(char_index + 3).unwrap() == 'S' {
                        count += 1;
                    }
                    // backward
                    if char_index >= 3
                        && line.chars().nth(char_index - 1).unwrap() == 'M'
                        && line.chars().nth(char_index - 2).unwrap() == 'A'
                        && line.chars().nth(char_index - 3).unwrap() == 'S' {
                        count += 1;
                    }
                    // down
                    if lines.len() - line_index >= 4
                        && lines[line_index + 1].chars().nth(char_index).unwrap() == 'M'
                        && lines[line_index + 2].chars().nth(char_index).unwrap() == 'A'
                        && lines[line_index + 3].chars().nth(char_index).unwrap() == 'S' {
                        count += 1;
                    }
                    // up
                    if line_index >= 3
                        && lines[line_index - 1].chars().nth(char_index).unwrap() == 'M'
                        && lines[line_index - 2].chars().nth(char_index).unwrap() == 'A'
                        && lines[line_index - 3].chars().nth(char_index).unwrap() == 'S' {
                        count += 1;
                    }
                    // diagonal down right
                    if lines.len() - line_index >= 4
                        && line.len() - char_index >= 4
                        && lines[line_index + 1].chars().nth(char_index + 1).unwrap() == 'M'
                        && lines[line_index + 2].chars().nth(char_index + 2).unwrap() == 'A'
                        && lines[line_index + 3].chars().nth(char_index + 3).unwrap() == 'S' {
                        count += 1;
                    }
                    // diagonal down left
                    if lines.len() - line_index >= 4
                        && char_index >= 3
                        && lines[line_index + 1].chars().nth(char_index - 1).unwrap() == 'M'
                        && lines[line_index + 2].chars().nth(char_index - 2).unwrap() == 'A'
                        && lines[line_index + 3].chars().nth(char_index - 3).unwrap() == 'S' {
                        count += 1;
                    }
                    // diagonal up right
                    if line_index >= 3
                        && line.len() - char_index >= 4
                        && lines[line_index - 1].chars().nth(char_index + 1).unwrap() == 'M'
                        && lines[line_index - 2].chars().nth(char_index + 2).unwrap() == 'A'
                        && lines[line_index - 3].chars().nth(char_index + 3).unwrap() == 'S' {
                        count += 1;
                    }
                    // diagonal up left
                    if line_index >= 3
                        && char_index >= 3
                        && lines[line_index - 1].chars().nth(char_index - 1).unwrap() == 'M'
                        && lines[line_index - 2].chars().nth(char_index - 2).unwrap() == 'A'
                        && lines[line_index - 3].chars().nth(char_index - 3).unwrap() == 'S' {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn part2(contents: &str) -> u64 {
        let lines: Vec<&str> = contents.lines().collect();
        let mut count = 0;
        for i in 1..lines.len() - 1 {
            for j in 1..lines[i].len() - 1 {
                let c = lines[i].chars().nth(j).unwrap();
                if c != 'A' {
                    continue;
                }
                let mut found = false;
                let up_left = lines[i - 1].chars().nth(j - 1).unwrap();
                let up_right = lines[i - 1].chars().nth(j + 1).unwrap();
                let down_left = lines[i + 1].chars().nth(j - 1).unwrap();
                let down_right = lines[i + 1].chars().nth(j + 1).unwrap();
                if up_left == 'M' && down_right == 'S' {
                    if (up_right == 'M' && down_left == 'S') || (up_right == 'S' && down_left == 'M') {
                        found = true;
                    }
                }
                else if up_left == 'S' && down_right == 'M' {
                    if (up_right == 'M' && down_left == 'S') || (up_right == 'S' && down_left == 'M') {
                        found = true;
                    }
                }
                else if up_right == 'M' && down_left == 'S' {
                    if (up_left == 'M' && down_right == 'S') || (up_left == 'S' && down_right == 'M') {
                        found = true;
                    }
                }
                else if up_right == 'S' && down_left == 'M' {
                    if (up_left == 'M' && down_right == 'S') || (up_left == 'S' && down_right == 'M') {
                        found = true;
                    }
                }
                if found {
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part1() {
    let contents = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    assert_eq!(Day04::part1(contents), 18);
  }

  #[test]
  fn test_part2() {
    let contents = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    assert_eq!(Day04::part2(contents), 9);
  }
}