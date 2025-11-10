use std::fs;
use std::path::Path;

static YEAR: &str = "2024";
static DAY: &str = "02";

fn load_data(prefix: Option<&str>, suffix: Option<&str>) -> Vec<Vec<isize>> {
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
        let mut report = Vec::new();
        for value in line.split(" ") {
            report.push(value.parse::<isize>().expect("Expect numerical value"));
        }
        data.push(report);
    }

    data
}

fn is_report_safe(report: Vec<isize>, tolerance: i8) -> bool {
    if tolerance < 0 {
        return false;
    }

    let mut latest_value = report[0];
    let direction: isize = if report[1] > report[0] { 1 } else { -1 };
    for (k, value) in report.iter().skip(1).enumerate() {
        let change = (value - latest_value) * direction;
        if change > 3 || change <= 0 {
            let mut option1 = report.clone();
            option1.remove(k);
            let mut option2 = report.clone();
            option2.remove(k + 1);
            let mut option3 = report.clone();
            option3.remove(0);

            return is_report_safe(option1, tolerance - 1)
                || is_report_safe(option2, tolerance - 1)
                || is_report_safe(option3, tolerance - 1);
        }
        latest_value = value.clone();
    }

    true
}

fn solve_part_1(prefix: Option<&str>, suffix: Option<&str>) -> usize {
    let data = load_data(prefix, suffix);
    let mut result: usize = 0;

    for report in data.iter() {
        if is_report_safe(report.to_vec(), 0) {
            result += 1;
        }
    }

    result
}

fn solve_part_2(prefix: Option<&str>, suffix: Option<&str>) -> usize {
    let mut result: usize = 0;
    let data = load_data(prefix, suffix);

    for report in data.iter() {
        if is_report_safe(report.to_vec(), 1) {
            result += 1;
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
        assert_eq!(result, 2);
    }

    #[test]
    fn answer_part_1() {
        let result = solve_part_1(Some("../"), None);
        assert_eq!(result, 524);
    }

    #[test]
    fn example_part_2() {
        let result = solve_part_2(Some("../"), Some(".example"));
        assert_eq!(result, 4);
    }

    #[test]
    fn answer_part_2() {
        let result = solve_part_2(Some("../"), None);
        assert_eq!(result, 569);
    }
}
