use regex::Regex;

pub fn part1(contents: &str) -> i32 {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let mults: Vec<&str> = re.captures_iter(contents).map(|x| x.get(0).unwrap().as_str()).collect();
    let mut sum = 0;
    mults.iter().for_each(|x| {
        let start = x.find("(").unwrap();
        let comma = x.find(",").unwrap();
        let end = x.find(")").unwrap();
        let a = x[start + 1..comma].parse::<i32>().unwrap();
        let b = x[comma + 1..end].parse::<i32>().unwrap();
        sum += a * b;
    });
    sum
}

pub fn part2(contents: &str) -> i32 {
    let re = Regex::new(r"don't\(\)|do\(\)|mul\(\d{1,3},\d{1,3}\)").unwrap();
    let mut enabled = true;
    let mut sum = 0;
    re.captures_iter(contents).for_each(|x| {
        let x = x.get(0).unwrap().as_str();
        if x == "do()" {
            enabled = true;
        } else if x == "don't()" {
            enabled = false;
        } else if enabled {
            let start = x.find("(").unwrap();
            let comma = x.find(",").unwrap();
            let end = x.find(")").unwrap();
            let a = x[start + 1..comma].parse::<i32>().unwrap();
            let b = x[comma + 1..end].parse::<i32>().unwrap();
            sum += a * b;
        }
    });
    sum
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part1() {
    let contents = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    assert_eq!(part1(contents), 161);
  }

  #[test]
  fn test_part2() {
    let contents = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    assert_eq!(part2(contents), 48);
  }
}