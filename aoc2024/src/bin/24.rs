use std::fs;
use std::path::Path;

static YEAR: &str = "2024";
static DAY: &str = "24";

fn load_data(prefix: Option<&str>, suffix: Option<&str>) -> () {
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

    // Process input data
}

fn solve_part_1(prefix: Option<&str>, suffix: Option<&str>) -> usize {
    let data = load_data(prefix, suffix);

    // Solve part 1

    0
}

fn solve_part_2(prefix: Option<&str>, suffix: Option<&str>) -> String {
    let data = load_data(prefix, suffix);

    // Solve part 2

    "wrong".to_string()
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
        assert_eq!(result, 2024);
    }

    #[test]
    fn answer_part_1() {
        let result = solve_part_1(Some("../"), None);
        assert_eq!(result, 42049478636360);
    }

    #[test]
    fn answer_part_2() {
        let result = solve_part_2(Some("../"), None);
        assert_eq!(result, "cph,gws,hgj,nnt,npf,z13,z19,z33");
    }
}
