use std::collections::{HashMap, HashSet};

pub fn part1(contents: &str) -> u32 {
    let y_max = contents.lines().count();
    let x_max = contents.lines().next().unwrap().chars().count();
    let antennas = parse_input(contents);
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    
    for (_, coords) in antennas.iter() {
        for i in 0..coords.len() {
            for j in i + 1..coords.len() {
                let (dist_x, dist_y) = (coords[i].0 as isize - coords[j].0 as isize, coords[i].1 as isize - coords[j].1 as isize);
                let (mut x, mut y) = (coords[i].0 as isize + dist_x, coords[i].1 as isize + dist_y);
                if x >= 0 && x < x_max as isize && y >= 0 && y < y_max as isize {
                    antinodes.insert((x as usize, y as usize));
                }
                (x, y) = (coords[j].0 as isize - dist_x, coords[j].1 as isize - dist_y);
                if x >= 0 && x < x_max as isize && y >= 0 && y < y_max as isize {
                    antinodes.insert((x as usize, y as usize));
                }
            }
        }
    }
    antinodes.len() as u32
}

pub fn part2(contents: &str) -> u32 {
    let y_max = contents.lines().count();
    let x_max = contents.lines().next().unwrap().chars().count();
    let antennas = parse_input(contents);
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    for (_, coords) in antennas.iter() {
        for i in 0..coords.len() {
            for j in i + 1..coords.len() {
                antinodes.insert(coords[i]);
                let (dist_x, dist_y) = (coords[i].0 as isize - coords[j].0 as isize, coords[i].1 as isize - coords[j].1 as isize);
                let (mut x, mut y) = (coords[i].0 as isize, coords[i].1 as isize);
                while x >= 0 && x < x_max as isize && y >= 0 && y < y_max as isize {
                    antinodes.insert((x as usize, y as usize));
                    x += dist_x;
                    y += dist_y;
                }
                x = coords[j].0 as isize;
                y = coords[j].1 as isize;
                while x >= 0 && x < x_max as isize && y >= 0 && y < y_max as isize {
                    antinodes.insert((x as usize, y as usize));
                    x -= dist_x;
                    y -= dist_y;
                }
            }
        }
    }

    antinodes.len() as u32
}

fn parse_input(contents: &str) -> HashMap<char, Vec<(usize, usize)>> {
    let mut map = HashMap::new();
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                let entry = map.entry(c).or_insert(Vec::new());
                entry.push((x, y));
            }
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(part1(contents), 14);
    }

    #[test]
    fn test_part2() {
        let contents = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(part2(contents), 34);
    }
}