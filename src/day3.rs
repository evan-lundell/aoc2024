use regex::Regex;

pub fn part1(contents: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(contents).map(|x| {
        let a = x.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let b = x.get(2).unwrap().as_str().parse::<i32>().unwrap();
        a * b
    }).sum()
}

pub fn part2(contents: &str) -> i32 {
    let re = Regex::new(r"do(n't)?\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut enabled = true;
    let mut sum = 0;
    re.captures_iter(contents).for_each(|x| {
        let command = x.get(0).unwrap().as_str();
        match command {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                if enabled {
                    let a = x.get(2).unwrap().as_str().parse::<i32>().unwrap();
                    let b = x.get(3).unwrap().as_str().parse::<i32>().unwrap();
                    sum += a * b;
                }
            }
        };
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