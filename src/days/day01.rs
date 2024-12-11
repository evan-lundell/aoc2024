use crate::days::Day;
use std::collections::HashMap;

pub struct Day01;

impl Day for Day01 {

    fn part1(contents: &str) -> u64 {
        let mut first = Vec::new();
        let mut second = Vec::new();
        for line in contents.lines() {
            let mut split = line.split_whitespace();
            first.push(split.next().unwrap().parse::<i32>().unwrap());
            second.push(split.next().unwrap().parse::<i32>().unwrap());
        }

        first.sort();
        second.sort();
        let mut sum: i32 = 0;
        for i in 0..first.len() {
            sum += (first[i] - second[i]).abs();
        }
        sum as u64
    }

    fn part2(contents: &str) -> u64 {
        let mut first = Vec::new();
        let mut second: HashMap<i32, i32> = HashMap::new();
        for line in contents.lines() {
            let mut split = line.split_whitespace();
            first.push(split.next().unwrap().parse::<i32>().unwrap());
            let count = second.entry(split.next().unwrap().parse::<i32>().unwrap()).or_insert(0);
            *count += 1;
        }

        let mut sum: i32 = 0;
        for i in first {
            if !second.contains_key(&i) {
                continue;
            }
            sum += i * second.get(&i).unwrap();
        }

        sum as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(Day01::part1(input), 11);
    }

    #[test]
    fn test_part2() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(Day01::part2(input), 31);
    }
}