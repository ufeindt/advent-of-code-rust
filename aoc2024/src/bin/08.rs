use itertools::iproduct;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

static YEAR: &str = "2024";
static DAY: &str = "08";

#[derive(Eq, Hash, PartialEq)]
struct Point {
    x: i16,
    y: i16,
}
struct AntennaMap {
    antennas: HashMap<char, Vec<Point>>,
    width: u8,
    height: u8,
}

fn load_data(prefix: Option<&str>, suffix: Option<&str>) -> AntennaMap {
    let mut file_name = format!("input/{YEAR}/{DAY}.input");
    match prefix {
        None => (),
        Some(p) => file_name = format!("{p}{file_name}"),
    }
    match suffix {
        None => (),
        Some(s) => file_name = format!("{file_name}{s}"),
    }

    let input =
        fs::read_to_string(Path::new(&file_name)).expect("Should have been able to read the file");

    let width = input.split("\n").next().unwrap().len() as u8;
    let height = input.split("\n").filter(|l| l.len() > 0).count() as u8;
    let mut antennas = HashMap::new();
    for (y, line) in input.split("\n").enumerate() {
        if line.len() == 0 {
            continue;
        }
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {}
                _ => {
                    if !antennas.contains_key(&c) {
                        antennas.insert(c, Vec::new());
                    }
                    antennas.get_mut(&c).unwrap().push(Point {
                        x: x as i16,
                        y: y as i16,
                    });
                }
            }
        }
    }

    AntennaMap {
        antennas,
        width,
        height,
    }
}

fn get_nodes(map: &AntennaMap, start: u8, limit: Option<u8>) -> HashSet<Point> {
    let mut nodes = HashSet::new();
    for locations in map.antennas.values() {
        for (location1, location2) in iproduct!(locations, locations) {
            if location1 == location2 {
                continue;
            }
            let direction = Point {
                x: location2.x - location1.x,
                y: location2.y - location1.y,
            };
            let mut step: u8 = start;
            loop {
                match limit {
                    None => (),
                    Some(l) => {
                        if step > l {
                            break;
                        }
                    }
                }
                let next = Point {
                    x: location2.x + direction.x * step as i16,
                    y: location2.y + direction.y * step as i16,
                };
                if next.x < 0
                    || next.x >= map.width as i16
                    || next.y < 0
                    || next.y >= map.height as i16
                {
                    break;
                }
                nodes.insert(next);
                step += 1;
            }
        }
    }

    nodes
}

fn solve_part_1(prefix: Option<&str>, suffix: Option<&str>) -> usize {
    let map = load_data(prefix, suffix);
    get_nodes(&map, 1, Some(1)).len()
}

fn solve_part_2(prefix: Option<&str>, suffix: Option<&str>) -> usize {
    let map = load_data(prefix, suffix);
    get_nodes(&map, 0, None).len()
}

fn main() {
    let answer1 = solve_part_1(None, None);
    println!("Answer for part 1: {answer1}");

    let answer2 = solve_part_2(None, None);
    println!("Answer for part 2: {answer2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        let result = solve_part_1(Some("../"), Some(".example"));
        assert_eq!(result, 14);
    }

    #[test]
    fn answer_part_1() {
        let result = solve_part_1(Some("../"), None);
        assert_eq!(result, 299);
    }

    #[test]
    fn example_part_2() {
        let result = solve_part_2(Some("../"), Some(".example"));
        assert_eq!(result, 34);
    }

    #[test]
    fn answer_part_2() {
        let result = solve_part_2(Some("../"), None);
        assert_eq!(result, 1032);
    }
}
