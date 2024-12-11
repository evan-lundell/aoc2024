use crate::days::Day;
use std::collections::HashSet;
use itertools::Itertools;

pub struct Day05;

impl Day for Day05 {

    fn part1 (contents: &str) -> u64 {
        let (rules, pages_index) = parse_input(contents);
        let mut sum = 0;
        contents.lines().skip(pages_index).map(|line| line.split(",").collect()).for_each(|pages: Vec<&str>| {
            let reverse_page_combo = pages.iter()
                .tuple_combinations::<(_, _)>()
                .map(|(a, b)| format!("{}|{}", b, a))
                .collect::<Vec<String>>();
            let mut valid = true;
            for combo in reverse_page_combo {
                if rules.contains(&combo) {
                    valid = false;
                    break;
                }
            }
            if valid {
                let half_index = pages.len() / 2;
                sum += pages.iter().nth(half_index).unwrap().parse::<u64>().unwrap();
            }
        });
        sum
    }

    fn part2 (contents: &str) -> u64 {
        let (rules, pages_index) = parse_input(contents);
        let mut sum = 0;
        contents.lines().skip(pages_index).map(|line| line.split(",").collect()).for_each(|mut pages: Vec<&str>| {
            let mut reverse_page_combo = pages.iter()
                .tuple_combinations::<(_, _)>()
                .map(|(a, b)| format!("{}|{}", b, a))
                .collect::<Vec<String>>();

            let mut fixed = false;
            let mut i = 0;
            while i < reverse_page_combo.len() {
                if !rules.contains(&reverse_page_combo[i]) {
                    i += 1;
                    continue;
                }
                fixed = true;
                let combo_clone = reverse_page_combo[i].clone();
                let split: Vec<&str> = combo_clone.split("|").collect();
                let index_of_first = pages.iter().position(|&x| x == split[0]).unwrap();
                let index_of_second = pages.iter().position(|&x| x == split[1]).unwrap();
                pages.swap(index_of_first, index_of_second);
                reverse_page_combo = pages.iter()
                    .tuple_combinations::<(_, _)>()
                    .map(|(a, b)| format!("{}|{}", b, a))
                    .collect::<Vec<String>>();
                i = 0;
            }
            if fixed {
                let half_index = pages.len() / 2;
                sum += pages.iter().nth(half_index).unwrap().parse::<u64>().unwrap();
            }
        });
        sum
    }
}

fn parse_input(contents: &str) -> (HashSet<String>, usize) {
    let mut set = HashSet::new();
    let mut pages_index = 0;
    for i in 0..contents.lines().count() {
        if contents.lines().nth(i).unwrap() == "" {
            pages_index = i + 1;
            break;
        }
        set.insert(contents.lines().nth(i).unwrap().to_string());
    }

    (set, pages_index)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        assert_eq!(Day05::part1(contents), 143);
    }

    #[test]
    fn test_part2() {
        let contents = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

            assert_eq!(Day05::part2(contents), 123);
    }
}
