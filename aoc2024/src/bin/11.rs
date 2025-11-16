use memoize::memoize;
use std::fs;
use std::path::Path;

static YEAR: &str = "2024";
static DAY: &str = "11";

fn load_data(prefix: Option<&str>, suffix: Option<&str>) -> Vec<usize> {
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
        .next()
        .unwrap()
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

#[memoize]
fn score(stone: usize, n_blinks: u8) -> usize {
    if n_blinks == 0 {
        return 1;
    }

    println!("stone: {stone} n_blinks: {n_blinks}");
    let stone_digits = if stone > 0 {
        (stone.ilog10() + 1) as u32
    } else {
        1
    };
    let new_stones = if stone == 0 {
        vec![1]
    } else if stone_digits % 2 == 0 {
        vec![
            stone % 10_usize.pow(stone_digits / 2),
            stone / 10_usize.pow(stone_digits / 2),
        ]
    } else {
        vec![stone * 2024]
    };

    new_stones
        .iter()
        .map(|new_stone| score(*new_stone, n_blinks - 1))
        .sum()
}

fn solve_part_1(prefix: Option<&str>, suffix: Option<&str>) -> usize {
    let stones = load_data(prefix, suffix);
    stones.iter().map(|stone| score(*stone, 25)).sum()
}

fn solve_part_2(prefix: Option<&str>, suffix: Option<&str>) -> usize {
    let stones = load_data(prefix, suffix);
    stones.iter().map(|stone| score(*stone, 75)).sum()
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
        assert_eq!(result, 55312);
    }

    #[test]
    fn answer_part_1() {
        let result = solve_part_1(Some("../"), None);
        assert_eq!(result, 203228);
    }

    #[test]
    fn answer_part_2() {
        let result = solve_part_2(Some("../"), None);
        assert_eq!(result, 240884656550923);
    }
}
