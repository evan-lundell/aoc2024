use std::collections::HashMap;

use crate::days::Day;

pub struct Day11;

impl Day for Day11 {
    fn part1(contents: &str) -> u64 {
        let mut stones: Vec<u64> = contents.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
        multiply_stones(&mut stones, 25)
    }

    fn part2(contents: &str) -> u64 {
        let stones: Vec<u64> = contents.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
        multiply_stones(&stones, 75)
    }
}

#[deprecated]
#[allow(dead_code)]
fn multiply_stones_old(stones: &mut Vec<u64>, blinks: u32) -> u64 {
    for _ in 0..blinks {    
        let mut i: usize = 0;
        while i < stones.len() {
            if stones[i] == 0 {
                stones[i] = 1;
                i += 1;
                continue;
            }

            let stone_string = stones[i].to_string();
            if stone_string.len() % 2 == 0 {
                stones[i] = stone_string[..stone_string.len() / 2].parse::<u64>().unwrap();
                stones.insert(i + 1, stone_string[stone_string.len() / 2..].parse::<u64>().unwrap());
                i += 2;
                continue;
            }

            stones[i] *= 2024;
            i += 1;
        }
    }

    let mut stones_count: HashMap<u64, u64> = HashMap::new();
    for stone in stones.iter() {
        let count = stones_count.entry(*stone).or_insert(0);
        *count += 1;
    }
    stones.len() as u64

}

fn multiply_stones(stones: &Vec<u64>, blinks: u32) -> u64 {
    let mut stones_count: HashMap<u64, u64> = HashMap::new();
    for stone in stones {
        let count = stones_count.entry(*stone).or_insert(0);
        *count += 1;
    }
    for _ in 0..blinks {
        let mut new_stones: HashMap<u64, u64> = HashMap::new();
        for (stone, count) in stones_count.iter_mut() {
            if *stone == 0 {
                let new_count = new_stones.entry(1).or_insert(0);
                *new_count += *count;
                continue;
            }

            let stone_string = stone.to_string();
            if stone_string.len() % 2 == 0 {
                let first_half = stone_string[..stone_string.len() / 2].parse::<u64>().unwrap();
                let second_half = stone_string[stone_string.len() / 2..].parse::<u64>().unwrap();
                let first_count = new_stones.entry(first_half).or_insert(0);
                *first_count += *count;
                let second_count = new_stones.entry(second_half).or_insert(0);
                *second_count += *count;
                continue;
            }

            let new_count = new_stones.entry(stone * 2024).or_insert(0);
            *new_count += *count;
        }
        stones_count = new_stones;
    }
    stones_count.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "125 17";
        assert_eq!(Day11::part1(input), 55312);
    }

    #[test]
    fn test_part2() {
        let input = "125 17";
        assert_eq!(Day11::part2(input), 65601038650482);
    }
}
