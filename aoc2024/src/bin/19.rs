use memoize::memoize;
use std::fs;
use std::path::Path;

static YEAR: &str = "2024";
static DAY: &str = "19";

fn convert_to_number(s: &str) -> usize {
    let mut result = 0;
    for c in s.chars() {
        result <<= 3;
        if c == 'w' {
            result |= 1;
        } else if c == 'u' {
            result |= 2;
        } else if c == 'b' {
            result |= 3;
        } else if c == 'r' {
            result |= 4;
        } else if c == 'g' {
            result |= 5;
        }
    }

    result
}

fn revert_to_string(n: usize) -> String {
    let mut result = String::new();
    let mut n = n;
    while n > 0 {
        let c = match n & 7 {
            1 => 'w',
            2 => 'u',
            3 => 'b',
            4 => 'r',
            5 => 'g',
            _ => unreachable!(),
        };
        result = format!("{c}{result}");
        n >>= 3;
    }
    result
}

fn get_bit_length(n: usize) -> usize {
    ((n.ilog2() / 3 + 1) * 3).try_into().unwrap()
}

fn load_data(prefix: Option<&str>, suffix: Option<&str>) -> (Vec<usize>, Vec<usize>) {
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
    let available = parts[0].split(", ").map(|s| convert_to_number(s)).collect();
    let desired = parts[1]
        .split("\n")
        .filter(|l| l.len() > 0)
        .map(|s| convert_to_number(s))
        .collect();

    (available, desired)
}

#[memoize]
fn find_combination(desired_pattern: usize, available_patterns: Vec<usize>) -> bool {
    for available in available_patterns.iter() {
        if desired_pattern < *available {
            continue;
        }

        if *available == desired_pattern {
            return true;
        }

        let bit_length = get_bit_length(*available);
        let end = desired_pattern - ((desired_pattern >> bit_length) << bit_length);
        if end == *available {
            if find_combination(desired_pattern >> bit_length, available_patterns.clone()) {
                return true;
            }
        }
    }

    false
}

#[memoize]
fn count_all_combinations(desired_pattern: usize, available_patterns: Vec<usize>) -> usize {
    let mut result: usize = 0;
    for available in available_patterns.iter() {
        if *available == desired_pattern {
            result += 1;
        }

        // if desired_pattern <= *available {
        //     continue;
        // }

        let bit_length = get_bit_length(*available);
        let end = desired_pattern - ((desired_pattern >> bit_length) << bit_length);
        if end == *available {
            result +=
                count_all_combinations(desired_pattern >> bit_length, available_patterns.clone());
        }
    }

    result
}

fn solve_part_1(prefix: Option<&str>, suffix: Option<&str>) -> usize {
    let (available, desired) = load_data(prefix, suffix);

    // for desired in desired.iter() {
    //     if !find_combination(*desired, available.clone()) {
    //         println!("{desired} not found");
    //     } else {
    //         println!("{desired} found");
    //     }
    // }
    // desired
    //     .iter()
    //     .filter(|d| find_combination(**d, available.clone()))
    //     .collect::<Vec<_>>()
    //     .len()
    desired
        .iter()
        .filter(|d| count_all_combinations(**d, available.clone()) > 0)
        .collect::<Vec<_>>()
        .len()
}

fn solve_part_2(prefix: Option<&str>, suffix: Option<&str>) -> usize {
    let (available, desired) = load_data(prefix, suffix);
    // for desired in desired.iter() {
    //     println!("{}", count_all_combinations(*desired, available.clone()));
    // }
    desired
        .iter()
        .map(|d| count_all_combinations(*d, available.clone()))
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

    // #[test]
    // fn convert() {
    //     assert_eq!(convert_to_number("wu"), 10);
    //     assert_eq!(convert_to_number("burg"), 1701);
    // }

    // #[test]
    // fn bit_length() {
    //     assert_eq!(get_bit_length(convert_to_number("w")), 3);
    //     assert_eq!(get_bit_length(convert_to_number("b")), 3);
    //     assert_eq!(get_bit_length(convert_to_number("u")), 3);
    //     assert_eq!(get_bit_length(convert_to_number("r")), 3);
    //     assert_eq!(get_bit_length(convert_to_number("g")), 3);
    //     assert_eq!(get_bit_length(convert_to_number("wu")), 6);
    //     assert_eq!(get_bit_length(convert_to_number("burg")), 12);
    //     assert_eq!(get_bit_length(convert_to_number("wuburg")), 18);
    // }

    // #[test]
    // fn shorten() {
    //     let desired = convert_to_number("wuburg");
    //     let available = convert_to_number("burg");
    //     assert_eq!(
    //         desired >> get_bit_length(available),
    //         convert_to_number("wu")
    //     );
    // }

    // #[test]
    // fn match_start() {
    //     let desired = convert_to_number("wuburg");
    //     let available = convert_to_number("wu");
    //     let not_available = convert_to_number("burg");
    //     assert_eq!(
    //         desired >> (get_bit_length(desired) - get_bit_length(available)),
    //         available
    //     );
    //     assert_ne!(
    //         desired >> (get_bit_length(desired) - get_bit_length(not_available)),
    //         not_available
    //     );
    // }

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
        assert_ne!(result, 16);
    }

    #[test]
    fn answer_part_2() {
        let result = solve_part_2(Some("../"), None);
        assert_eq!(result, 666491493769758);
    }
}
