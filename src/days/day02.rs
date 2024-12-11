use crate::days::Day;

pub struct Day02;

impl Day for Day02 {

    fn part1(contents: &str) -> u64 {
        let mut safe_count = 0;
        for line in contents.lines() {
            let levels = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            if is_level_safe(&levels) {
                safe_count += 1;
            }
        }
        safe_count
    }

    fn part2(contents: &str) -> u64 {
        let mut safe_count = 0;
        for line in contents.lines() {
            let levels = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            let mut safe: bool = false;
            for i in 0..levels.len() {
                let mut levels_copy = levels.clone();
                levels_copy.remove(i);
                if is_level_safe(&levels_copy) {
                    safe = true;
                    break;
                }
            }
            if safe {
                safe_count += 1;
            }
        }
        safe_count
    }
}

fn is_level_safe(levels: &Vec<i32>) -> bool {
    let increasing = levels[0] < levels[1];
    for i in 0..levels.len() - 1 {
        let diff = levels[i] - levels[i + 1];
        if diff == 0 {
            return false;
        }
        if (increasing && diff > 0) || (!increasing && diff < 0) {
            return false;
        }
        let diff_abs = diff.abs();
        if diff_abs > 3 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";
        assert_eq!(Day02::part1(contents), 2);
    }

    #[test]
    fn test_part2() {
        let contents = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";
        assert_eq!(Day02::part2(contents), 4);
    }
}