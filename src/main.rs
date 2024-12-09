use std::fs;
mod day7;

fn main() {
    let contents = fs::read_to_string("src/input/day7.txt")
        .expect("Something went wrong reading the file");
    println!("Day 7 Part 1: {}", day7::part1(&contents));
    println!("Day 7 Part 2: {}", day7::part2(&contents));
}
