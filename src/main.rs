use std::fs;
mod day9;

fn main() {
    let contents = fs::read_to_string("src/input/day9.txt")
        .expect("Something went wrong reading the file");
    println!("Day 9 Part 1: {}", day9::part1(&contents));
    println!("Day 9 Part 2: {}", day9::part2(&contents));
}
