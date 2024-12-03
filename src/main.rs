use std::fs;
mod day3;

fn main() {
    let contents = fs::read_to_string("src/input/day3.txt")
        .expect("Something went wrong reading the file");
    println!("Day 2 Part 1: {}", day3::part1(&contents));
    println!("Day 2 Part 2: {}", day3::part2(&contents));
}
