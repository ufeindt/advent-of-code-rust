use regex::Regex;
use std::fs;
use std::path::Path;

static YEAR: &str = "2024";
static DAY: &str = "14";

#[derive(Copy, Clone, Debug)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Copy, Clone, Debug)]
struct Robot {
    position: Point,
    velocity: Point,
}

impl Robot {
    fn move_robot(&mut self, dt: isize, width: isize, height: isize) {
        self.position.x = (self.position.x + self.velocity.x * dt).rem_euclid(width);
        self.position.y = (self.position.y + self.velocity.y * dt).rem_euclid(height);
    }
}

fn load_data(prefix: Option<&str>, suffix: Option<&str>) -> Vec<Robot> {
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
    let mut robots = Vec::new();
    for line in input.split("\n") {
        if line.len() == 0 {
            continue;
        }

        let re = Regex::new(r"p=([\-0-9]+),([\-0-9]+) v=([\-0-9]+),([\-0-9]+)").unwrap();
        let captures = re.captures(line).unwrap();
        robots.push(Robot {
            position: Point {
                x: captures[1].parse::<isize>().unwrap(),
                y: captures[2].parse::<isize>().unwrap(),
            },
            velocity: Point {
                x: captures[3].parse::<isize>().unwrap(),
                y: captures[4].parse::<isize>().unwrap(),
            },
        });
    }

    robots
}

fn safety_factor(robots: &Vec<Robot>, width: isize, height: isize) -> usize {
    println!("{} {}", width / 2, height / 2);
    println!("{:?}", robots);
    let sectors = vec![
        robots
            .iter()
            .filter(|robot| robot.position.x < width / 2 && robot.position.y < height / 2)
            .count(),
        robots
            .iter()
            .filter(|robot| robot.position.x > width / 2 && robot.position.y < height / 2)
            .count(),
        robots
            .iter()
            .filter(|robot| robot.position.x < width / 2 && robot.position.y > height / 2)
            .count(),
        robots
            .iter()
            .filter(|robot| robot.position.x > width / 2 && robot.position.y > height / 2)
            .count(),
    ];

    println!("{:?}", sectors);
    sectors.iter().product()
}

fn variance(robots: &Vec<Robot>) -> (f32, f32) {
    let mean_x: f32 = robots
        .iter()
        .map(|robot| robot.position.x as f32)
        .sum::<f32>()
        / robots.len() as f32;
    let mean_y: f32 = robots
        .iter()
        .map(|robot| robot.position.y as f32)
        .sum::<f32>()
        / robots.len() as f32;

    let var_x = robots
        .iter()
        .map(|robot| (robot.position.x as f32 - mean_x).powi(2))
        .sum::<f32>()
        / robots.len() as f32;
    let var_y = robots
        .iter()
        .map(|robot| (robot.position.y as f32 - mean_y).powi(2))
        .sum::<f32>()
        / robots.len() as f32;
    (var_x.sqrt(), var_y.sqrt())
}

fn solve_part_1(prefix: Option<&str>, suffix: Option<&str>, width: isize, height: isize) -> usize {
    let mut robots = load_data(prefix, suffix);
    for robot in robots.iter_mut() {
        robot.move_robot(100, width, height);
    }

    safety_factor(&robots, width, height)
}

fn solve_part_2(prefix: Option<&str>, suffix: Option<&str>, width: isize, height: isize) -> usize {
    let mut robots = load_data(prefix, suffix);
    let mut variances_x = vec![];
    let mut variances_y = vec![];

    let max_dim = [width, height].into_iter().max().unwrap();
    for _ in 0..max_dim {
        for robot in robots.iter_mut() {
            robot.move_robot(1, width, height);
        }
        let (var_x, var_y) = variance(&robots);
        variances_x.push(var_x);
        variances_y.push(var_y);
    }

    let min_var_x = variances_x.iter().fold(f32::INFINITY, |a, &b| a.min(b));
    let min_var_y = variances_y.iter().fold(f32::INFINITY, |a, &b| a.min(b));

    let min_var_x_index = variances_x.iter().position(|&x| x == min_var_x).unwrap() + 1;
    let min_var_y_index = variances_y.iter().position(|&y| y == min_var_y).unwrap() + 1;

    let x_steps = (0..max_dim)
        .map(|k| min_var_x_index + k as usize * width as usize)
        .collect::<Vec<usize>>();
    let mut result = min_var_y_index;
    for _ in 0..max_dim {
        result += height as usize;
        if x_steps.contains(&result) {
            return result;
        }
    }

    0
}

fn main() {
    let answer1 = solve_part_1(None, None, 101, 103);
    println!("Answer for part 1: {answer1}");

    let answer2 = solve_part_2(None, None, 101, 103);
    println!("Answer for part 2: {answer2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        let result = solve_part_1(Some("../"), Some(".example"), 11, 7);
        assert_eq!(result, 12);
    }

    #[test]
    fn answer_part_1() {
        let result = solve_part_1(Some("../"), None, 101, 103);
        assert_eq!(result, 211773366);
    }

    #[test]
    fn answer_part_2() {
        let result = solve_part_2(Some("../"), None, 101, 103);
        assert_eq!(result, 7344);
    }
}
