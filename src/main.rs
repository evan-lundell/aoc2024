use std::fs;
mod day5;

fn main() {
    let contents = fs::read_to_string("src/input/day5.txt")
        .expect("Something went wrong reading the file");
    println!("Day 5 Part 1: {}", day5::part1(&contents));
    println!("Day 5 Part 2: {}", day5::part2(&contents));
}
