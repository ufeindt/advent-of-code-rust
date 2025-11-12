use std::collections::HashSet;
use std::fs;
use std::path::Path;

static YEAR: &str = "2024";
static DAY: &str = "06";

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i16,
    y: i16,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Guard {
    facing: Direction,
    position: Point,
}

enum PathResult {
    Exited(HashSet<Guard>),
    Loop,
}
struct Map {
    width: i16,
    height: i16,
    obstacles: HashSet<Point>,
}

fn load_data(prefix: Option<&str>, suffix: Option<&str>) -> (Map, Guard) {
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

    let mut map = Map {
        width: 0,
        height: 0,
        obstacles: HashSet::new(),
    };
    let mut guard = Guard {
        facing: Direction::Right,
        position: Point { x: 0, y: 0 },
    };
    for (y, line) in input.split("\n").enumerate() {
        if line.len() == 0 {
            continue;
        }
        for (x, c) in line.chars().enumerate() {
            let position = Point {
                x: x as i16,
                y: y as i16,
            };
            match c {
                '.' => {}
                '#' => {
                    map.obstacles.insert(position);
                }
                '^' => {
                    guard.facing = Direction::Up;
                    guard.position = position;
                }
                'v' => {
                    guard.facing = Direction::Down;
                    guard.position = position;
                }
                '<' => {
                    guard.facing = Direction::Left;
                    guard.position = position;
                }
                '>' => {
                    guard.facing = Direction::Right;
                    guard.position = position;
                }
                _ => panic!("Unexpected character"),
            }
            if y == 0 {
                map.width += 1;
            }
        }
        map.height += 1;
    }

    (map, guard)
}

fn move_guard(guard: Guard, map: &Map) -> Option<Guard> {
    let mut new_guard = guard.clone();
    match new_guard.facing {
        Direction::Up => new_guard.position.y -= 1,
        Direction::Right => new_guard.position.x += 1,
        Direction::Down => new_guard.position.y += 1,
        Direction::Left => new_guard.position.x -= 1,
    }

    if new_guard.position.x < 0
        || new_guard.position.y < 0
        || new_guard.position.x >= map.width as i16
        || new_guard.position.y >= map.height as i16
    {
        return None;
    }

    if map.obstacles.contains(&new_guard.position) {
        match new_guard.facing {
            Direction::Up => new_guard.facing = Direction::Right,
            Direction::Right => new_guard.facing = Direction::Down,
            Direction::Down => new_guard.facing = Direction::Left,
            Direction::Left => new_guard.facing = Direction::Up,
        }
        new_guard.position = Point {
            x: guard.position.x,
            y: guard.position.y,
        };
    }

    Some(new_guard)
}

fn get_guard_path(mut guard: Guard, map: &Map) -> PathResult {
    let mut path = HashSet::new();
    path.insert(guard.clone());
    loop {
        let new_guard = move_guard(guard.clone(), map);
        match new_guard {
            None => return PathResult::Exited(path),
            Some(g) => {
                if path.contains(&g) {
                    return PathResult::Loop;
                }
                path.insert(g.clone());
                guard = g.clone();
            }
        }
    }
}

fn get_guard_visited(guard: Guard, map: &Map) -> HashSet<Point> {
    match get_guard_path(guard, map) {
        PathResult::Exited(p) => p.iter().map(|g| g.position).collect(),
        PathResult::Loop => panic!("Guard is in a loop"),
    }
}

fn solve_part_1(prefix: Option<&str>, suffix: Option<&str>) -> usize {
    let (map, guard) = load_data(prefix, suffix);
    let visited = get_guard_visited(guard, &map);

    visited.len()
}

fn solve_part_2(prefix: Option<&str>, suffix: Option<&str>) -> usize {
    let mut result: usize = 0;
    let (map, guard) = load_data(prefix, suffix);
    let visited = get_guard_visited(guard.clone(), &map);

    for point in visited.iter() {
        let mut new_obstacles = map.obstacles.clone();
        new_obstacles.insert(*point);
        let new_map = Map {
            width: map.width,
            height: map.height,
            obstacles: new_obstacles,
        };
        match get_guard_path(guard.clone(), &new_map) {
            PathResult::Loop => result += 1,
            _ => {}
        }
    }

    result
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
        assert_eq!(result, 41);
    }

    #[test]
    fn answer_part_1() {
        let result = solve_part_1(Some("../"), None);
        assert_eq!(result, 5208);
    }

    #[test]
    fn example_part_2() {
        let result = solve_part_2(Some("../"), Some(".example"));
        assert_eq!(result, 6);
    }

    #[test]
    fn answer_part_2() {
        let result = solve_part_2(Some("../"), None);
        assert_eq!(result, 1972);
    }
}
