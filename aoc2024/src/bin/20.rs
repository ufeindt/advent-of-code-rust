use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

static YEAR: &str = "2024";
static DAY: &str = "20";

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i16,
    y: i16,
}

struct Maze {
    walls: HashSet<Point>,
    start: Point,
    end: Point,
}

fn load_data(prefix: Option<&str>, suffix: Option<&str>) -> Maze {
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

    let mut maze = Maze {
        walls: HashSet::new(),
        start: Point { x: 0, y: 0 },
        end: Point { x: 0, y: 0 },
    };
    for (y, line) in input.split("\n").enumerate() {
        if line.len() == 0 {
            continue;
        }
        for (x, c) in line.chars().enumerate() {
            let point = Point {
                x: x as i16,
                y: y as i16,
            };
            match c {
                '#' => {
                    maze.walls.insert(point);
                }
                'S' => {
                    maze.start = point;
                }
                'E' => {
                    maze.end = point;
                }
                _ => {}
            }
        }
    }

    maze
}

fn find_path(maze: &Maze) -> Vec<Point> {
    let mut queue = vec![(maze.start.clone(), vec![maze.start.clone()])];
    let mut visited = HashSet::from([maze.start.clone()]);
    while queue.len() > 0 {
        let (current, path) = queue.pop().unwrap();
        if current == maze.end {
            return path;
        }

        for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let next_coords = Point {
                x: current.x + dx,
                y: current.y + dy,
            };
            if !visited.contains(&next_coords) && !maze.walls.contains(&next_coords) {
                visited.insert(next_coords.clone());
                let mut new_path = path.clone();
                new_path.push(next_coords.clone());
                queue.insert(0, (next_coords, new_path));
            }
        }
    }

    vec![]
}

fn find_cheats(maze: &Maze, min_dt: i16, max_dist_cheat: i16) -> HashMap<(Point, Point), i16> {
    let directions = vec![(-1, 0), (0, -1), (1, 0), (0, 1)];
    let mut cheats = HashMap::new();
    let mut path_map = HashMap::new();
    for (t, coords) in find_path(maze).iter().enumerate() {
        path_map.insert(coords.clone(), t as i16);
    }

    for (coords, t) in path_map.iter() {
        let mut secondary_directions;
        for (dx_main, dy_main) in directions.iter() {
            if *dx_main == 0 {
                secondary_directions = vec![(-1, 0), (1, 0)];
            } else {
                secondary_directions = vec![(0, -1), (0, 1)];
            }

            for (dx_secondary, dy_secondary) in secondary_directions.iter() {
                for k_main in 1..=max_dist_cheat {
                    for k_secondary in 0..=(max_dist_cheat - k_main) {
                        let cheat_coords = Point {
                            x: coords.x
                                + (k_main as i16) * dx_main
                                + (k_secondary as i16) * dx_secondary,
                            y: coords.y
                                + (k_main as i16) * dy_main
                                + (k_secondary as i16) * dy_secondary,
                        };
                        if path_map.contains_key(&cheat_coords) {
                            let t_cheat = path_map[&cheat_coords];
                            let dist = (cheat_coords.x - coords.x).abs()
                                + (cheat_coords.y - coords.y).abs();
                            let dt = t_cheat - t - dist;

                            if dist <= max_dist_cheat && dt >= min_dt {
                                cheats.insert((coords.clone(), cheat_coords.clone()), dt);
                            }
                        }
                    }
                }
            }
        }
    }

    cheats
}

fn solve(prefix: Option<&str>, suffix: Option<&str>, min_dt: i16, max_dist_cheat: i16) -> usize {
    let maze = load_data(prefix, suffix);
    let cheats = find_cheats(&maze, min_dt, max_dist_cheat);

    cheats.len()
}

fn main() {
    let answer1 = solve(None, None, 100, 2);
    println!("Answer for part 1: {answer1}");

    let answer2 = solve(None, None, 100, 20);
    println!("Answer for part 2: {answer2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        assert_eq!(solve(Some("../"), Some(".example"), 64, 2), 1);
        assert_eq!(solve(Some("../"), Some(".example"), 40, 2), 2);
        assert_eq!(solve(Some("../"), Some(".example"), 38, 2), 3);
        assert_eq!(solve(Some("../"), Some(".example"), 36, 2), 4);
        assert_eq!(solve(Some("../"), Some(".example"), 20, 2), 5);
        assert_eq!(solve(Some("../"), Some(".example"), 12, 2), 8);
        assert_eq!(solve(Some("../"), Some(".example"), 10, 2), 10);
        assert_eq!(solve(Some("../"), Some(".example"), 8, 2), 14);
        assert_eq!(solve(Some("../"), Some(".example"), 6, 2), 16);
        assert_eq!(solve(Some("../"), Some(".example"), 4, 2), 30);
        assert_eq!(solve(Some("../"), Some(".example"), 2, 2), 44);
    }

    #[test]
    fn answer_part_1() {
        let result = solve(Some("../"), None, 100, 2);
        assert_eq!(result, 1263);
    }

    #[test]
    fn example_part_2() {
        assert_eq!(solve(Some("../"), Some(".example"), 76, 20), 3);
        assert_eq!(solve(Some("../"), Some(".example"), 74, 20), 7);
        assert_eq!(solve(Some("../"), Some(".example"), 72, 20), 29);
        assert_eq!(solve(Some("../"), Some(".example"), 70, 20), 41);
        assert_eq!(solve(Some("../"), Some(".example"), 68, 20), 55);
        assert_eq!(solve(Some("../"), Some(".example"), 66, 20), 67);
        assert_eq!(solve(Some("../"), Some(".example"), 64, 20), 86);
        assert_eq!(solve(Some("../"), Some(".example"), 62, 20), 106);
        assert_eq!(solve(Some("../"), Some(".example"), 60, 20), 129);
        assert_eq!(solve(Some("../"), Some(".example"), 58, 20), 154);
        assert_eq!(solve(Some("../"), Some(".example"), 56, 20), 193);
        assert_eq!(solve(Some("../"), Some(".example"), 54, 20), 222);
        assert_eq!(solve(Some("../"), Some(".example"), 52, 20), 253);
        assert_eq!(solve(Some("../"), Some(".example"), 50, 20), 285);
    }

    #[test]
    fn answer_part_2() {
        let result = solve(Some("../"), None, 100, 20);
        assert_eq!(result, 957831);
    }
}
