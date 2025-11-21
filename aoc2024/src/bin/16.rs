use std::collections::HashSet;
use std::path::Path;
use std::{fs, vec};

static YEAR: &str = "2024";
static DAY: &str = "16";

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Facing {
    North,
    East,
    South,
    West,
}

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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct RouteStep {
    point: Point,
    facing: Facing,
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

fn turn_left(facing: &Facing) -> Facing {
    match facing {
        Facing::North => Facing::West,
        Facing::East => Facing::North,
        Facing::South => Facing::East,
        Facing::West => Facing::South,
    }
}

fn turn_right(facing: &Facing) -> Facing {
    match facing {
        Facing::North => Facing::East,
        Facing::East => Facing::South,
        Facing::South => Facing::West,
        Facing::West => Facing::North,
    }
}

fn get_step(facing: &Facing) -> Point {
    match facing {
        Facing::North => Point { x: 0, y: -1 },
        Facing::East => Point { x: 1, y: 0 },
        Facing::South => Point { x: 0, y: 1 },
        Facing::West => Point { x: -1, y: 0 },
    }
}

fn find_best_routes(maze: &Maze) -> Vec<(Vec<Vec<RouteStep>>, u32)> {
    let mut queue: Vec<(RouteStep, Vec<Vec<RouteStep>>, u32)> = vec![(
        RouteStep {
            point: maze.start.clone(),
            facing: Facing::East,
        },
        vec![vec![]],
        0,
    )];
    let mut visited: HashSet<RouteStep> = HashSet::new();
    let mut routes: Vec<(Vec<Vec<RouteStep>>, u32)> = vec![];

    loop {
        if queue.len() == 0 {
            break;
        }
        let (current, route, score) = queue.pop().unwrap();

        if current.point == maze.end {
            routes.push((route, score));
            continue;
        }

        let step = get_step(&current.facing);
        let options = [
            (
                RouteStep {
                    point: Point {
                        x: current.point.x + step.x,
                        y: current.point.y + step.y,
                    },
                    facing: current.facing.clone(),
                },
                score + 1,
            ),
            (
                RouteStep {
                    point: current.point.clone(),
                    facing: turn_left(&current.facing),
                },
                score + 1000,
            ),
            (
                RouteStep {
                    point: current.point.clone(),
                    facing: turn_right(&current.facing),
                },
                score + 1000,
            ),
        ];

        for (next_route_step, next_score) in options {
            if maze.walls.contains(&next_route_step.point) {
                continue;
            }
            if visited.contains(&next_route_step) {
                let matching_queue_index = queue
                    .iter()
                    .position(|(r, _, s)| *r == next_route_step && s == &next_score);
                match matching_queue_index {
                    None => (),
                    Some(index) => {
                        let mut new = route.clone();
                        for new_option in new.iter_mut() {
                            new_option.push(next_route_step.clone());
                        }
                        queue[index].1.extend(new);
                    }
                }
                continue;
            }

            let mut new_route = route.clone();
            for route_option in new_route.iter_mut() {
                route_option.push(next_route_step.clone());
            }
            visited.insert(next_route_step.clone());
            queue.push((next_route_step, new_route, next_score));
        }
        queue.sort_by_key(|(_, _, score)| *score);
        queue.reverse();
    }

    let min_score = routes.iter().map(|(_, score)| score).min().unwrap();
    routes
        .iter()
        .filter(|(_, score)| score == min_score)
        .map(|(route, score)| (route.clone(), *score))
        .collect()
}

fn print_maze_visited(maze: &Maze, visited: &HashSet<Point>) {
    let width = maze.walls.iter().map(|p| p.x).max().unwrap() + 1;
    let height = maze.walls.iter().map(|p| p.y).max().unwrap() + 1;
    for y in 0..height {
        for x in 0..width {
            let point = Point { x, y };
            if visited.contains(&point) {
                print!("O");
            } else if maze.walls.contains(&point) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn solve_part_1(prefix: Option<&str>, suffix: Option<&str>) -> u32 {
    let maze = load_data(prefix, suffix);
    let routes = find_best_routes(&maze);
    *routes.iter().map(|(_, score)| score).min().unwrap()
}

fn solve_part_2(prefix: Option<&str>, suffix: Option<&str>) -> usize {
    let maze = load_data(prefix, suffix);
    let routes = find_best_routes(&maze);
    println!("{}", routes.len());

    let mut visited = HashSet::new();
    for (route, _) in routes {
        for route_options in route {
            for route_step in route_options {
                visited.insert(route_step.point);
            }
        }
    }
    print_maze_visited(&maze, &visited);

    visited.len()
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
    fn example1_part_1() {
        let result = solve_part_1(Some("../"), Some(".example"));
        assert_eq!(result, 7036);
    }

    #[test]
    fn example2_part_1() {
        let result = solve_part_1(Some("../"), Some(".example2"));
        assert_eq!(result, 11048);
    }

    #[test]
    fn answer_part_1() {
        let result = solve_part_1(Some("../"), None);
        assert_eq!(result, 98484);
    }

    #[test]
    fn example1_part_2() {
        let result = solve_part_2(Some("../"), Some(".example"));
        assert_eq!(result, 45);
    }

    #[test]

    fn example2_part_2() {
        let result = solve_part_2(Some("../"), Some(".example2"));
        assert_eq!(result, 64);
    }

    #[test]
    fn answer_part_2() {
        let result = solve_part_2(Some("../"), None);
        assert_eq!(result, 531);
    }
}
