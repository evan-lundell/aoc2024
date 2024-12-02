use std::collections::HashMap;

pub fn part1(contents: &str) -> i32 {
    let mut first = Vec::new();
    let mut second = Vec::new();
    for line in contents.lines() {
        let mut split = line.split_whitespace();
        first.push(split.next().unwrap().parse::<i32>().unwrap());
        second.push(split.next().unwrap().parse::<i32>().unwrap());
    }

    first.sort();
    second.sort();
    let mut sum: i32 = 0;
    for i in 0..first.len() {
        sum += (first[i] - second[i]).abs();
    }
    sum
}

pub fn part2(contents: &str) -> i32 {
    let mut first = Vec::new();
    let mut second: HashMap<i32, i32> = HashMap::new();
    for line in contents.lines() {
        let mut split = line.split_whitespace();
        first.push(split.next().unwrap().parse::<i32>().unwrap());
        let count = second.entry(split.next().unwrap().parse::<i32>().unwrap()).or_insert(0);
        *count += 1;
    }

    let mut sum: i32 = 0;
    for i in first {
        if !second.contains_key(&i) {
            continue;
        }
        sum += i * second.get(&i).unwrap();
    }

    sum
}