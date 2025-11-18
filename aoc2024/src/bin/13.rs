use regex::Regex;
use std::path::Path;
use std::{fs, vec};

static YEAR: &str = "2024";
static DAY: &str = "13";

#[derive(Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Copy, Clone)]
struct Machine {
    a: Point,
    b: Point,
    p: Point,
}

fn load_data(prefix: Option<&str>, suffix: Option<&str>) -> Vec<Machine> {
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

    let re = Regex::new(r"X[\=\+]([0-9]+), Y[\=\+]([0-9]+)").unwrap();
    let mut a_values = vec![];
    let mut b_values = vec![];
    let mut p_values = vec![];
    for line in input.split("\n") {
        if line.len() == 0 {
            continue;
        }
        let captures = re.captures(line).unwrap();
        let new_point = Point {
            x: captures[1].parse::<i64>().unwrap(),
            y: captures[2].parse::<i64>().unwrap(),
        };
        if line.starts_with("Button A") {
            a_values.push(new_point);
        } else if line.starts_with("Button B") {
            b_values.push(new_point);
        } else if line.starts_with("Prize") {
            p_values.push(new_point);
        }
    }

    let mut machines = vec![];
    for ((a, b), p) in a_values.iter().zip(b_values.iter()).zip(p_values.iter()) {
        machines.push(Machine {
            a: *a,
            b: *b,
            p: *p,
        })
    }

    machines
}

fn find_machine_cost(machine: Machine) -> i64 {
    let det = machine.a.x * machine.b.y - machine.a.y * machine.b.x;
    let d_a = machine.b.y * machine.p.x - machine.b.x * machine.p.y;
    let d_b = machine.a.x * machine.p.y - machine.a.y * machine.p.x;
    if d_a % det == 0 && d_b % det == 0 {
        let n_a = d_a / det;
        let n_b = d_b / det;
        if n_a > 0 && n_b > 0 {
            return 3 * n_a + n_b;
        }
    }
    0
}

fn solve_part_1(prefix: Option<&str>, suffix: Option<&str>) -> i64 {
    let machines = load_data(prefix, suffix);
    machines
        .iter()
        .map(|machine| find_machine_cost(*machine))
        .sum()
}

fn solve_part_2(prefix: Option<&str>, suffix: Option<&str>) -> i64 {
    let machines = load_data(prefix, suffix);
    machines
        .iter()
        .map(|machine| {
            find_machine_cost(Machine {
                a: machine.a,
                b: machine.b,
                p: Point {
                    x: machine.p.x + 10_000_000_000_000,
                    y: machine.p.y + 10_000_000_000_000,
                },
            })
        })
        .sum()
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
        assert_eq!(result, 480);
    }

    #[test]
    fn answer_part_1() {
        let result = solve_part_1(Some("../"), None);
        assert_eq!(result, 31897);
    }

    #[test]
    fn answer_part_2() {
        let result = solve_part_2(Some("../"), None);
        assert_eq!(result, 87596249540359);
    }
}
