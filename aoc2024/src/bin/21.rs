use memoize::memoize;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

static YEAR: &str = "2024";
static DAY: &str = "21";

fn load_data(prefix: Option<&str>, suffix: Option<&str>) -> Vec<String> {
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

    input
        .split("\n")
        .filter(|l| l.len() > 0)
        .map(|s| s.to_string())
        .collect()
}

#[memoize]
fn get_numerical_direction_options(start: String, end: String) -> HashSet<String> {
    let keys = HashMap::from([
        (String::from("0"), (1, 3)),
        (String::from("1"), (0, 2)),
        (String::from("2"), (1, 2)),
        (String::from("3"), (2, 2)),
        (String::from("4"), (0, 1)),
        (String::from("5"), (1, 1)),
        (String::from("6"), (2, 1)),
        (String::from("7"), (0, 0)),
        (String::from("8"), (1, 0)),
        (String::from("9"), (2, 0)),
        (String::from("A"), (2, 3)),
    ]);
    let mut keys_locations = HashSet::new();
    for (_, v) in keys.iter() {
        keys_locations.insert(*v);
    }

    let (start_x, start_y) = keys.get(&start).unwrap();
    let (end_x, end_y) = keys.get(&end).unwrap();

    let dx = end_x - start_x;
    let dy = end_y - start_y;
    let x_operation = (if dx > 0 { ">" } else { "<" }).repeat((dx as i32).abs() as usize);
    let y_operation = (if dy > 0 { "v" } else { "^" }).repeat((dy as i32).abs() as usize);

    let mut options = HashSet::new();
    if keys_locations.contains(&(start_x + dx, *start_y)) {
        options.insert(format!("{x_operation}{y_operation}A"));
    }
    if keys_locations.contains(&(*start_x, start_y + dy)) {
        options.insert(format!("{y_operation}{x_operation}A"));
    }

    options
}

#[memoize]
fn get_directional_keypress_count(start: String, end: String, n_robots: u8) -> usize {
    if n_robots == 1 {
        return 1;
    }
    let keys = HashMap::from([
        (String::from("^"), (1, 0)),
        (String::from("v"), (1, 1)),
        (String::from("<"), (0, 1)),
        (String::from(">"), (2, 1)),
        (String::from("A"), (2, 0)),
    ]);
    let mut keys_locations = HashSet::new();
    for (_, v) in keys.iter() {
        keys_locations.insert(*v);
    }

    let (start_x, start_y) = keys.get(&start).unwrap();
    let (end_x, end_y) = keys.get(&end).unwrap();

    let dx = end_x - start_x;
    let dy = end_y - start_y;
    let x_operation = (if dx > 0 { ">" } else { "<" }).repeat((dx as i32).abs() as usize);
    let y_operation = (if dy > 0 { "v" } else { "^" }).repeat((dy as i32).abs() as usize);

    let mut options = HashSet::new();
    if keys_locations.contains(&(start_x + dx, *start_y)) {
        options.insert(format!("{x_operation}{y_operation}A"));
    }
    if keys_locations.contains(&(*start_x, start_y + dy)) {
        options.insert(format!("{y_operation}{x_operation}A"));
    }

    let mut counts = vec![];
    for option in options {
        let mut count = 0;
        for (start, end) in format!("A{}", option).chars().zip(option.chars()) {
            count +=
                get_directional_keypress_count(start.to_string(), end.to_string(), n_robots - 1);
        }
        counts.push(count);
    }

    *counts.iter().min().unwrap()
}

fn get_human_keypress_count(code: &str, n_robots: u8) -> usize {
    let mut keypress_options = vec![];
    for (start, end) in format!("A{}", code).chars().zip(code.chars()) {
        keypress_options.push(get_numerical_direction_options(
            start.to_string(),
            end.to_string(),
        ));
    }

    let mut total = 0;
    for options in keypress_options {
        let mut option_counts = vec![];
        for option in options {
            let mut count = 0;
            for (start, end) in format!("A{}", option).chars().zip(option.chars()) {
                count +=
                    get_directional_keypress_count(start.to_string(), end.to_string(), n_robots);
            }
            option_counts.push(count);
        }
        total += option_counts.iter().min().unwrap();
    }

    total
}

fn solve(prefix: Option<&str>, suffix: Option<&str>, n_robots: u8) -> usize {
    let codes = load_data(prefix, suffix);
    codes
        .iter()
        .map(|c| get_human_keypress_count(c, n_robots) * c[..c.len() - 1].parse::<usize>().unwrap())
        .sum()
}

fn main() {
    let answer1 = solve(None, None, 3);
    println!("Answer for part 1: {answer1}");

    let answer2 = solve(None, None, 26);
    println!("Answer for part 2: {answer2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        let result = solve(Some("../"), Some(".example"), 3);
        assert_eq!(result, 126384);
    }

    #[test]
    fn answer_part_1() {
        let result = solve(Some("../"), None, 3);
        assert_eq!(result, 134120);
    }

    #[test]
    fn answer_part_2() {
        let result = solve(Some("../"), None, 26);
        assert_eq!(result, 167389793580400);
    }
}
