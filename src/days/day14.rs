use std::io;
use std::thread;
use std::time::Duration;
use crate::days::Day;
use crossterm::cursor::MoveTo;
use regex::Regex;
use crossterm::{execute, terminal::{Clear, ClearType}};

pub struct Day14;

struct Robot {
    position: (i64, i64),
    velocity: (i64, i64)
}

impl Day for Day14 {
    fn part1(contents: &str) -> u64 {
        let grid_size = (101, 103);
        // let grid_size = (11, 7);
        let half_grid_size = (grid_size.0 / 2, grid_size.1 / 2);
        let mut quad_counts = vec![0; 4];
        let robots = parse_input(contents);
        let new_robot = find_robots_at_frame(&robots, 100, grid_size);

        for robot in new_robot.iter() {
            if robot.position.0 < half_grid_size.0 && robot.position.1 < half_grid_size.1 {
                quad_counts[0] += 1;
            } else if robot.position.0 > half_grid_size.0 && robot.position.1 < half_grid_size.1 {
                quad_counts[1] += 1;
            } else if robot.position.0 < half_grid_size.0 && robot.position.1 > half_grid_size.1 {
                quad_counts[2] += 1;
            } else if robot.position.0 > half_grid_size.0 && robot.position.1 > half_grid_size.1 {
                quad_counts[3] += 1;
            }
        }
        
        quad_counts[0] * quad_counts[1] * quad_counts[2] * quad_counts[3]
    }

    fn part2(_contents: &str) -> u64 {
        let grid_size = (101, 103);
        let robots = parse_input(_contents);
        let mut frame = 18;
        loop {
            frame += 103;
            let new_robots = find_robots_at_frame(&robots, frame, grid_size);
            execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
            println!("Frame: {}", frame);
            print_robots(&new_robots, grid_size);
            thread::sleep(Duration::from_millis(200));
            if frame == 8258 {
                break;
            }
        }
        0
    }
}

fn parse_input(contents: &str) -> Vec<Robot> {
    let mut robots: Vec<Robot> = Vec::new();
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    for cap in re.captures_iter(contents) {
        let x = cap[1].parse().unwrap();
        let y = cap[2].parse().unwrap();
        let vx = cap[3].parse().unwrap();
        let vy = cap[4].parse().unwrap();
        robots.push(Robot {
            position: (x, y),
            velocity: (vx, vy)
        });
    }
    robots
}

fn find_robots_at_frame(robots: &Vec<Robot>, frame: i64, grid_size: (i64, i64)) -> Vec<Robot> {
    let mut new_robots: Vec<Robot> = Vec::new();
    for robot in robots.iter() {
        let mut x = (robot.position.0 + (robot.velocity.0 * frame)) % grid_size.0;
        if x < 0 {
            x = grid_size.0 + x;
        }
        let mut y = (robot.position.1 + (robot.velocity.1 * frame)) % grid_size.1;
        if y < 0 {
            y = grid_size.1 + y;
        }

        new_robots.push(Robot {
            position: (x, y),
            velocity: robot.velocity
        });
    }
    new_robots
}

fn print_robots(robots: &Vec<Robot>, grid_size: (i64, i64)) {
    let mut grid = vec![vec!['.'; grid_size.0 as usize]; grid_size.1 as usize];
    for robot in robots.iter() {
        grid[robot.position.1 as usize][robot.position.0 as usize] = '#';
    }

    for row in grid.iter() {
        for cell in row.iter() {
            print!("{}", cell);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!(Day14::part1(input), 21);
    }

    // Day 2 part 2 test is not implemented because it's a visual test
}
