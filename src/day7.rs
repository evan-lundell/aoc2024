pub fn part1(contents: &str) -> u64 {
    let mut matches: u64 = 0;
    let input: Vec<(u64, Vec<u64>)> = contents
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let result = parts.next().unwrap().parse().unwrap();
            let values = parts.next().unwrap().split(" ").map(|x| x.parse().unwrap()).collect();
            (result, values)
        })
        .collect();

    input.iter().for_each(|(result, values)| {
        if do_values_match_result(*result, values) {
            matches += *result;
        }
    });
    matches
}

pub fn part2(contents: &str) -> u64 {
    let mut matches = 0;
    let input: Vec<(u64, Vec<u64>)> = contents
    .lines()
    .map(|line| {
        let mut parts = line.split(": ");
        let result = parts.next().unwrap().parse().unwrap();
        let values = parts.next().unwrap().split(" ").map(|x| x.parse().unwrap()).collect();
        (result, values)
    })
    .collect();

    input.iter().for_each(|(result, values)| {
        if do_values_match_result_with_or(*result, values, 1, values[0]) {
            matches += *result;
        }
    });
    matches
}

fn do_values_match_result(result: u64, values: &Vec<u64>) -> bool {
    for i in 0..(2u32.pow(values.len() as u32 - 1)) {
        let mut total = values[0];
        for j in 0..values.len() - 1 {
            if is_bit_set(i, j as u32) {
                total *= values[j + 1];
            } else {
                total += values[j + 1];
            }
        }
        if total == result {
            return true;
        }
    }
    false
}

fn is_bit_set(n: u32, position: u32) -> bool {
    n & (1 << position) != 0
}

fn do_values_match_result_with_or(result: u64, values: &Vec<u64>, index: usize, current_total: u64) -> bool {
    if index == values.len() {
        return current_total == result;
    }

    let next_value = values[index];

    // Try adding
    if do_values_match_result_with_or(result, values, index + 1, current_total + next_value) {
        return true;
    }

    // Try multiplying
    if do_values_match_result_with_or(result, values, index + 1, current_total * next_value) {
        return true;
    }

    // Try concatenating
    let concatenated_value = combine_numbers(current_total, next_value);
    if do_values_match_result_with_or(result, values, index + 1, concatenated_value) {
        return true;
    }

    false
}

fn combine_numbers(a: u64, b: u64) -> u64 {
    let combined_string = format!("{}{}", a, b);
    combined_string.parse::<u64>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
      assert_eq!(part1("190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"), 3749);
  }

    #[test]
    fn test_part2() {
      assert_eq!(part2("190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"), 11387);
    }
}
