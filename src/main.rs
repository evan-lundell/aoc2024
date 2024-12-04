use std::fs;
mod day4;

fn main() {
    let contents = fs::read_to_string("src/input/day4.txt")
        .expect("Something went wrong reading the file");
    println!("Day 2 Part 1: {}", day4::part1(&contents));
    println!("Day 2 Part 2: {}", day4::part2(&contents));
}
