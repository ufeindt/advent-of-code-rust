use std::fs;
use std::path::Path;

static YEAR: &str = "2024";
static DAY: &str = "25";

fn load_data(prefix: Option<&str>, suffix: Option<&str>) -> (Vec<[u8; 5]>, Vec<[u8; 5]>) {
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

    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for lock_or_key in input.split("\n\n") {
        let is_lock = lock_or_key.starts_with("#####");
        let mut colums = [0 as u8; 5];
        let mut rows = lock_or_key.lines().collect::<Vec<_>>();
        if !is_lock {
            rows.reverse();
        }
        for row in rows.iter().skip(1) {
            for (i, c) in row.chars().enumerate() {
                if c == '#' {
                    colums[i] += 1;
                }
            }
        }
        if is_lock {
            locks.push(colums);
        } else {
            keys.push(colums);
        }
    }

    (locks, keys)
}

fn solve_part_1(prefix: Option<&str>, suffix: Option<&str>) -> usize {
    let (locks, keys) = load_data(prefix, suffix);

    let mut total = 0;
    for lock in locks.iter() {
        for key in keys.iter() {
            if lock.iter().zip(key.iter()).all(|(l, k)| l + k <= 5) {
                total += 1;
            }
        }
    }

    total
}

fn main() {
    let answer1 = solve_part_1(None, None);
    println!("Answer for part 1: {answer1}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        let result = solve_part_1(Some("../"), Some(".example"));
        assert_eq!(result, 3);
    }

    #[test]
    fn answer_part_1() {
        let result = solve_part_1(Some("../"), None);
        assert_eq!(result, 2835);
    }
}
