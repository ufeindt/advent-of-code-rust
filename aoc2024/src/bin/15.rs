use std::collections::HashSet;
use std::fs;
use std::path::Path;

static YEAR: &str = "2024";
static DAY: &str = "15";

#[derive(PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i8,
    y: i8,
}

struct Map {
    current: Point,
    boxes: HashSet<Point>,
    obstacles: HashSet<Point>,
    doubled: bool,
}

impl Map {
    fn move_robot(&mut self, direction: &Direction) {
        if self.doubled {
            self.move_robot_double(direction);
        } else {
            self.move_robot_single(direction);
        }
    }

    fn move_robot_single(&mut self, direction: &Direction) {
        let (mut dx, mut dy) = (0 as i8, 0 as i8);
        match direction {
            Direction::Up => dy = -1,
            Direction::Right => dx = 1,
            Direction::Down => dy = 1,
            Direction::Left => dx = -1,
        }

        let mut empty_space_found = false;
        let (mut x, mut y) = (self.current.x, self.current.y);
        loop {
            x = x + dx;
            y = y + dy;
            if self.obstacles.contains(&Point { x, y }) {
                break;
            } else if self.boxes.contains(&Point { x, y }) {
                continue;
            }
            empty_space_found = true;
            break;
        }
        if empty_space_found {
            self.current = Point {
                x: self.current.x + dx,
                y: self.current.y + dy,
            };
            self.boxes.insert(Point { x, y });
            self.boxes.remove(&self.current);
        }
    }

    fn move_robot_double(&mut self, direction: &Direction) {
        let (mut dx, mut dy) = (0 as i8, 0 as i8);
        match direction {
            Direction::Up => dy = -1,
            Direction::Right => dx = 1,
            Direction::Down => dy = 1,
            Direction::Left => dx = -1,
        }

        let mut boxes = HashSet::new();
        let mut to_check = HashSet::from([Point {
            x: self.current.x + dx,
            y: self.current.y + dy,
        }]);
        let mut empty_space_found = false;
        loop {
            let obstacles_found = self.obstacles.intersection(&to_check).count() > 0;
            if obstacles_found {
                break;
            }
            let mut new_boxes = HashSet::new();
            for p in to_check.iter() {
                if self.boxes.contains(p) {
                    new_boxes.insert(Point { x: p.x, y: p.y });
                } else if self.boxes.contains(&Point { x: p.x - 1, y: p.y }) {
                    new_boxes.insert(Point { x: p.x - 1, y: p.y });
                }
            }
            if new_boxes.len() > 0 {
                let mut new_to_check = HashSet::new();
                if dy == 0 {
                    new_to_check.extend(new_boxes.iter().map(|p| Point {
                        x: p.x + (if dx > 0 { 2 } else { -1 }),
                        y: p.y,
                    }));
                } else {
                    for p in new_boxes.iter() {
                        new_to_check.insert(Point {
                            x: p.x,
                            y: p.y + dy,
                        });
                        new_to_check.insert(Point {
                            x: p.x + 1,
                            y: p.y + dy,
                        });
                    }
                }
                boxes.extend(new_boxes);
                to_check = new_to_check;
                continue;
            }
            empty_space_found = true;
            break;
        }
        if empty_space_found {
            self.current = Point {
                x: self.current.x + dx,
                y: self.current.y + dy,
            };
            for b in boxes.iter() {
                self.boxes.remove(&b);
            }
            for b in boxes.iter() {
                self.boxes.insert(Point {
                    x: b.x + dx,
                    y: b.y + dy,
                });
            }
        }
    }

    fn sum_gps(&self) -> usize {
        self.boxes
            .iter()
            .fold(0, |acc, p| acc + p.x as usize + 100 * p.y as usize)
    }
}
fn load_data(prefix: Option<&str>, suffix: Option<&str>, doubled: bool) -> (Map, Vec<Direction>) {
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

    let parts: Vec<&str> = input.split("\n\n").collect();
    let mut map = Map {
        current: Point { x: 0, y: 0 },
        boxes: HashSet::new(),
        obstacles: HashSet::new(),
        doubled,
    };

    for (y, line) in parts[0].split("\n").enumerate() {
        if line.len() == 0 {
            continue;
        }
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    if !doubled {
                        map.obstacles.insert(Point {
                            x: x as i8,
                            y: y as i8,
                        });
                    } else {
                        map.obstacles.insert(Point {
                            x: 2 * x as i8,
                            y: y as i8,
                        });
                        map.obstacles.insert(Point {
                            x: (2 * x + 1) as i8,
                            y: y as i8,
                        });
                    }
                }
                '@' => {
                    map.current = Point {
                        x: if doubled { 2 * x } else { x } as i8,
                        y: y as i8,
                    };
                }
                'O' => {
                    map.boxes.insert(Point {
                        x: if doubled { 2 * x } else { x } as i8,
                        y: y as i8,
                    });
                }
                _ => {}
            }
        }
    }

    let mut directions: Vec<Direction> = vec![];
    for c in parts[1].chars() {
        let new_directions = match c {
            '^' => Some(Direction::Up),
            '>' => Some(Direction::Right),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            _ => None,
        };
        match new_directions {
            None => (),
            Some(d) => directions.push(d),
        }
    }

    (map, directions)
}

fn solve_part_1(prefix: Option<&str>, suffix: Option<&str>) -> usize {
    let (mut map, directions) = load_data(prefix, suffix, false);
    for direction in directions {
        map.move_robot(&direction);
    }
    map.sum_gps()
}

fn solve_part_2(prefix: Option<&str>, suffix: Option<&str>) -> usize {
    let (mut map, directions) = load_data(prefix, suffix, true);
    for direction in directions {
        map.move_robot(&direction);
    }
    map.sum_gps()
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
        assert_eq!(result, 10092);
    }

    #[test]
    fn answer_part_1() {
        let result = solve_part_1(Some("../"), None);
        assert_eq!(result, 1442192);
    }

    #[test]
    fn example_part_2() {
        let result = solve_part_2(Some("../"), Some(".example"));
        assert_eq!(result, 9021);
    }

    #[test]
    fn answer_part_2() {
        let result = solve_part_2(Some("../"), None);
        assert_eq!(result, 1448458);
    }
}
