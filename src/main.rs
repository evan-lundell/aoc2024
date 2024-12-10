use std::fs;
mod day10;

fn main() {
    let contents = fs::read_to_string("src/input/day10.txt")
        .expect("Something went wrong reading the file");
    println!("Day 9 Part 1: {}", day10::part1(&contents));
    println!("Day 9 Part 2: {}", day10::part2(&contents));
}
