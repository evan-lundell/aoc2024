use std::fs;
mod day1;

fn main() {
    let contents = fs::read_to_string("src/input/day1.txt")
        .expect("Something went wrong reading the file");
    let part1_result = day1::part1(&contents);
    println!("Part1 Result: {}", part1_result);

    let part2_result = day1::part2(&contents);
    println!("Part2 Result: {}", part2_result);
}
