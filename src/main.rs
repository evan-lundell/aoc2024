use std::fs;
mod day6;

fn main() {
    let contents = fs::read_to_string("src/input/day6.txt")
        .expect("Something went wrong reading the file");
    println!("Day 6 Part 1: {}", day6::part1(&contents));
    println!("Day 6 Part 2: {}", day6::part2(&contents));
}
