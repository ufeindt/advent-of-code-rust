use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

static YEAR: &str = "2024";
static DAY: &str = "22";

fn load_data(prefix: Option<&str>, suffix: Option<&str>) -> Vec<isize> {
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
        .map(|s| s.parse::<isize>().unwrap())
        .collect()
}

fn next_secret(mut secret: isize) -> isize {
    secret ^= secret << 6;
    secret %= 1 << 24;
    secret ^= secret >> 5;
    secret %= 1 << 24;
    secret ^= secret << 11;
    secret %= 1 << 24;

    secret
}

fn solve_part_1(prefix: Option<&str>, suffix: Option<&str>) -> isize {
    let secrets = load_data(prefix, suffix);
    secrets
        .iter()
        .map(|secret| (0..2000).fold(*secret, |acc, _| next_secret(acc)))
        .sum()
}

fn solve_part_2(prefix: Option<&str>, suffix: Option<&str>) -> isize {
    let secrets = load_data(prefix, suffix);

    let mut sequence_totals = HashMap::new();
    for secret in secrets {
        let mut old_secret = secret;
        let mut encountered = HashSet::new();
        let mut current_sequence = (0, 0, 0, 0);
        for k in 0..2000 {
            let new_secret = next_secret(old_secret);
            current_sequence = (
                current_sequence.1,
                current_sequence.2,
                current_sequence.3,
                (new_secret % 10) - (old_secret % 10),
            );
            // println!("{k}: {:?}", current_sequence);
            if k >= 3 && !encountered.contains(&current_sequence) {
                encountered.insert(current_sequence);
                let total = sequence_totals.entry(current_sequence).or_insert(0);
                *total += new_secret % 10;
            }
            old_secret = new_secret;
        }
    }

    *sequence_totals.values().max().unwrap()
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
        assert_eq!(result, 37327623);
    }

    #[test]
    fn answer_part_1() {
        let result = solve_part_1(Some("../"), None);
        assert_eq!(result, 14273043166);
    }

    #[test]
    fn example_part_2() {
        let result = solve_part_2(Some("../"), Some(".example2"));
        assert_eq!(result, 23);
    }

    #[test]
    fn answer_part_2() {
        let result = solve_part_2(Some("../"), None);
        assert_eq!(result, 1667);
    }
}
