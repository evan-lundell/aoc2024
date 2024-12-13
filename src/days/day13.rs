use crate::days::Day;

pub struct Day13;

struct MachineInfo {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64)
}

impl Day for Day13 {
    fn part1(contents: &str) -> u64 {
        let machines = parse_input(contents);
        let mut total: i64 = 0;
        for machine in machines.iter() {
            match solve_machine(machine, 0) {
                Some((a, b)) => total += (a * 3) + b,
                None => continue
            }
        }
        
        total as u64
    }

    fn part2(contents: &str) -> u64 {
        let machines = parse_input(contents);
        let mut total: i64 = 0;
        for machine in machines.iter() {
            match solve_machine(machine, 10000000000000) {
                Some((a, b)) => total += (a * 3) + b,
                None => continue
            }
        }
        
        total as u64
    }
}

fn parse_input(contents: &str) -> Vec<MachineInfo> {
    let mut machines: Vec<MachineInfo> = Vec::new();
    let lines: Vec<&str> = contents.lines().collect();
    for chunk in lines.chunks(4) {
        let button_a = parse_coordinates(chunk[0]);
        let button_b = parse_coordinates(chunk[1]);
        let prize = parse_coordinates(chunk[2]);
        machines.push(MachineInfo {
            button_a,
            button_b,
            prize,
        });
    }
    machines
}

fn parse_coordinates(line: &str) -> (i64, i64) {
    let parts: Vec<&str> = line.split(&['+', '=', ','][..]).collect();
    let x = parts[1].trim().parse().unwrap();
    let y = parts[3].trim().parse().unwrap();
    (x, y)
}

fn solve_machine(machine: &MachineInfo, offset: i64) -> Option<(i64, i64)> {
    let prize = (machine.prize.0 + offset, machine.prize.1 + offset);
    let det = (machine.button_a.0 * machine.button_b.1) - (machine.button_a.1 * machine.button_b.0);
    if det == 0 {
        return None;
    }
    let a = ((prize.0 * machine.button_b.1) - (prize.1 * machine.button_b.0)) / det;
    let b = ((machine.button_a.0 * prize.1) - (machine.button_a.1 * prize.0)) / det;
    if (machine.button_a.0 * a + machine.button_b.0 * b, machine.button_a.1 * a + machine.button_b.1 * b) == prize {
        return Some((a, b));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!(Day13::part1(input), 480);
    }

    #[test]
    fn test_part2() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!(Day13::part2(input), 875318608908);
    }
}
