use std::collections::HashSet;
use std::fs;
use std::path::Path;

static YEAR: &str = "2024";
static DAY: &str = "18";

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    x: i8,
    y: i8,
}

fn load_data(prefix: Option<&str>, suffix: Option<&str>) -> Vec<Point> {
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

    let mut obstacles = vec![];
    for line in input.split("\n") {
        if line.len() == 0 {
            continue;
        }
        let line_split: Vec<&str> = line.split(",").collect();
        obstacles.push(Point {
            x: line_split[0].parse().unwrap(),
            y: line_split[1].parse().unwrap(),
        })
    }

    obstacles
}

fn find_path(obstacles: &[Point]) -> Option<Vec<Point>> {
    let mut queue = vec![(Point { x: 0, y: 0 }, vec![])];
    let mut visited = HashSet::from([Point { x: 0, y: 0 }]);

    let goal = Point {
        x: obstacles.iter().map(|p| p.x).max().unwrap(),
        y: obstacles.iter().map(|p| p.y).max().unwrap(),
    };

    while queue.len() > 0 {
        let (current, path) = queue.pop().unwrap();
        if current == goal {
            return Some(path);
        }

        let neighbors = vec![
            Point {
                x: current.x + 1,
                y: current.y,
            },
            Point {
                x: current.x - 1,
                y: current.y,
            },
            Point {
                x: current.x,
                y: current.y + 1,
            },
            Point {
                x: current.x,
                y: current.y - 1,
            },
        ];

        for neighbor in neighbors {
            if neighbor.x >= 0
                && neighbor.y >= 0
                && neighbor.x <= goal.x
                && neighbor.y <= goal.y
                && !visited.contains(&neighbor)
                && !obstacles.contains(&neighbor)
            {
                let mut new_path = path.clone();
                new_path.push(neighbor);
                queue.insert(0, (neighbor, new_path));
                visited.insert(neighbor);
            }
        }
    }

    None
}

fn solve_part_1(prefix: Option<&str>, suffix: Option<&str>, n_fallen: usize) -> usize {
    let obstacles = load_data(prefix, suffix);
    let path = find_path(&obstacles[..n_fallen]);
    match path {
        None => panic!("No path found"),
        Some(p) => p.len(),
    }
}

fn solve_part_2(prefix: Option<&str>, suffix: Option<&str>, n_fallen: usize) -> String {
    let obstacles = load_data(prefix, suffix);

    let mut min_n = n_fallen;
    let mut max_n = obstacles.len() + 1;
    let mut mid_n = 0;
    while max_n - min_n > 1 {
        mid_n = (min_n + max_n) / 2;
        match find_path(&obstacles[..mid_n]) {
            None => max_n = mid_n,
            Some(_) => min_n = mid_n,
        }
    }

    let first_blocker = obstacles[mid_n - 1];
    format!("{},{}", first_blocker.x, first_blocker.y)
}

fn main() {
    let answer1 = solve_part_1(None, None, 1024);
    println!("Answer for part 1: {answer1}");

    let answer2 = solve_part_2(None, None, 1024);
    println!("Answer for part 2: {answer2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        let result = solve_part_1(Some("../"), Some(".example"), 12);
        assert_eq!(result, 22);
    }

    #[test]
    fn answer_part_1() {
        let result = solve_part_1(Some("../"), None, 1024);
        assert_eq!(result, 310);
    }

    #[test]
    fn example_part_2() {
        let result = solve_part_2(Some("../"), Some(".example"), 12);
        assert_eq!(result, "6,1");
    }

    #[test]
    fn answer_part_2() {
        let result = solve_part_2(Some("../"), None, 1024);
        assert_eq!(result, "16,46");
    }
}
