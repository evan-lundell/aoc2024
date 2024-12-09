use std::fs;
mod day8;

fn main() {
    let contents = fs::read_to_string("src/input/day8.txt")
        .expect("Something went wrong reading the file");
    println!("Day 8 Part 1: {}", day8::part1(&contents));
    println!("Day 8 Part 2: {}", day8::part2(&contents));
}
