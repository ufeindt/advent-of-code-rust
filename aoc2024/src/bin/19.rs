use memoize::memoize;
use std::fs;
use std::path::Path;

static YEAR: &str = "2024";
static DAY: &str = "19";

fn load_data(prefix: Option<&str>, suffix: Option<&str>) -> (Vec<String>, Vec<String>) {
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
    let available = parts[0].split(", ").map(|s| s.to_string()).collect();
    let desired = parts[1]
        .split("\n")
        .filter(|l| l.len() > 0)
        .map(|s| s.to_string())
        .collect();

    (available, desired)
}

#[memoize]
fn find_combination(
    desired_pattern: String,
    available_patterns: Vec<String>,
) -> Option<Vec<String>> {
    for available in available_patterns.iter() {
        if *available == desired_pattern {
            return Some(vec![available.clone()]);
        }

        if desired_pattern.starts_with(available) {
            if let Some(combination) = find_combination(
                desired_pattern[available.len()..].to_string(),
                available_patterns.clone(),
            ) {
                let mut new_combination = vec![available.clone()];
                new_combination.extend(combination);
                return Some(new_combination);
            }
        }
    }

    None
}

#[memoize]
fn count_all_combinations(desired_pattern: String, available_patterns: Vec<String>) -> usize {
    let mut result: usize = 0;
    for available in available_patterns.iter() {
        if *available == desired_pattern {
            result += 1;
        }

        if desired_pattern.starts_with(available) {
            result += count_all_combinations(
                desired_pattern[available.len()..].to_string(),
                available_patterns.clone(),
            )
        }
    }

    result
}

fn solve_part_1(prefix: Option<&str>, suffix: Option<&str>) -> usize {
    let (available, desired) = load_data(prefix, suffix);

    desired
        .iter()
        .filter(|d| find_combination(d.to_string(), available.clone()).is_some())
        .collect::<Vec<_>>()
        .len()
}

fn solve_part_2(prefix: Option<&str>, suffix: Option<&str>) -> usize {
    let (available, desired) = load_data(prefix, suffix);
    desired
        .iter()
        .map(|d| count_all_combinations(d.clone(), available.clone()))
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
        assert_eq!(result, 6);
    }

    #[test]
    fn answer_part_1() {
        let result = solve_part_1(Some("../"), None);
        assert_eq!(result, 313);
    }

    #[test]
    fn example_part_2() {
        let result = solve_part_2(Some("../"), Some(".example"));
        assert_eq!(result, 16);
    }

    #[test]
    fn answer_part_2() {
        let result = solve_part_2(Some("../"), None);
        assert_eq!(result, 666491493769758);
    }
}
