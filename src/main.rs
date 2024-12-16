// main.rs
use std::{env, path::PathBuf};
use std::fs;

mod days;

use days::*;

type DayFn = fn(&str) -> u64;

fn run_day(input: &str, part1: DayFn, part2: DayFn) {
    println!("Part 1: \n{}", part1(input));
    println!("Part 2: \n{}", part2(input));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <day>", args[0]);
        return;
    }

    let day: u32 = args[1].parse().expect("Invalid day");

   // Construct the file path to the input file
   let mut input_path = PathBuf::from("input");
   input_path.push(format!("day{:02}.txt", day));

    // Read the file contents
    let contents = fs::read_to_string(input_path)
        .expect("Something went wrong reading the file");

    match args[1].as_str() {
        "01" => run_day(&contents, day01::Day01::part1, day01::Day01::part2),
        "02" => run_day(&contents, day02::Day02::part1, day02::Day02::part2),
        "03" => run_day(&contents, day03::Day03::part1, day03::Day03::part2),
        "04" => run_day(&contents, day04::Day04::part1, day04::Day04::part2),
        "05" => run_day(&contents, day05::Day05::part1, day05::Day05::part2),
        "06" => run_day(&contents, day06::Day06::part1, day06::Day06::part2),
        "07" => run_day(&contents, day07::Day07::part1, day07::Day07::part2),
        "08" => run_day(&contents, day08::Day08::part1, day08::Day08::part2),
        "09" => run_day(&contents, day09::Day09::part1, day09::Day09::part2),
        "10" => run_day(&contents, day10::Day10::part1, day10::Day10::part2),
        "11" => run_day(&contents, day11::Day11::part1, day11::Day11::part2),
        "12" => run_day(&contents, day12::Day12::part1, day12::Day12::part2),
        "13" => run_day(&contents, day13::Day13::part1, day13::Day13::part2),
        "14" => run_day(&contents, day14::Day14::part1, day14::Day14::part2),
        "15" => run_day(&contents, day15::Day15::part1, day15::Day15::part2),
        // Add more matches here
        _ => eprintln!("Unknown day: {}", args[1]),
    }
}
