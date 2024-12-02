use std::fs;
mod day1;
mod day2;

fn main() {
    let contents = fs::read_to_string("src/input/day2.txt")
        .expect("Something went wrong reading the file");
    println!("Day 2 Part 1: {}", day2::part1(&contents));
    println!("Day 2 Part 2: {}", day2::part2(&contents));
}
