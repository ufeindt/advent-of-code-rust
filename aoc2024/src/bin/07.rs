use std::path::Path;
use std::{fs, vec};

static YEAR: &str = "2024";
static DAY: &str = "07";

#[derive(Clone, Copy, Debug)]
enum Operation {
    Add,
    Multiply,
    Concatenate,
}

fn load_data(prefix: Option<&str>, suffix: Option<&str>) -> Vec<(usize, Vec<usize>)> {
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

    let mut data = Vec::new();
    for line in input.split("\n") {
        if line.len() == 0 {
            continue;
        }
        let mut parts = line.split(": ");
        let target = parts.next().unwrap().parse::<usize>().unwrap();
        let values = parts
            .next()
            .unwrap()
            .split(" ")
            .map(|v| v.parse::<usize>().unwrap())
            .collect();
        data.push((target, values));
    }

    data
}

fn validate(target: usize, values: &[usize], operations: Vec<Operation>) -> bool {
    if values.len() == 1 {
        if values[0] == target {
            return true;
        }
    } else if values.len() > 1 {
        for operation in operations.clone().iter() {
            let value = values.last().unwrap();
            let new_target: usize;
            // println!("target: {target} values: {values:?} operation: {operation:?}");
            match operation {
                Operation::Add => {
                    if *value > target {
                        continue;
                    }
                    new_target = target - value;
                }
                Operation::Multiply => {
                    if target % *value != 0 {
                        continue;
                    }
                    new_target = target / value;
                }
                Operation::Concatenate => {
                    let next_power_of_10 = 10_usize.pow((value.ilog10() + 1) as u32);
                    if (*value > target) || ((target - *value) % next_power_of_10 != 0) {
                        continue;
                    }
                    new_target = (target - value) / next_power_of_10;
                }
            }
            if validate(new_target, &values[..values.len() - 1], operations.clone()) {
                return true;
            }
        }
    }
    false
}

fn solve_part_1(prefix: Option<&str>, suffix: Option<&str>) -> usize {
    let mut result: usize = 0;
    let data = load_data(prefix, suffix);

    for (target, values) in data.iter() {
        if validate(*target, values, vec![Operation::Add, Operation::Multiply]) {
            result += target;
        }
    }

    result
}

fn solve_part_2(prefix: Option<&str>, suffix: Option<&str>) -> usize {
    let mut result: usize = 0;
    let data = load_data(prefix, suffix);

    for (target, values) in data.iter() {
        if validate(
            *target,
            values,
            vec![Operation::Add, Operation::Multiply, Operation::Concatenate],
        ) {
            result += target;
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
        assert_eq!(result, 3749);
    }

    #[test]
    fn answer_part_1() {
        let result = solve_part_1(Some("../"), None);
        assert_eq!(result, 12940396350192);
    }

    #[test]
    fn example_part_2() {
        let result = solve_part_2(Some("../"), Some(".example"));
        assert_eq!(result, 11387);
    }

    #[test]
    fn answer_part_2() {
        let result = solve_part_2(Some("../"), None);
        assert_eq!(result, 106016735664498);
    }
}
