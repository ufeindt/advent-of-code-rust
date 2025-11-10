use regex::Regex;
use std::fs;
use std::path::Path;

static YEAR: &str = "2024";
static DAY: &str = "03";

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

    input.split("\n").map(|s| s.to_string()).collect()
}

fn solve_part_1(prefix: Option<&str>, suffix: Option<&str>) -> usize {
    let mut result: usize = 0;
    let data = load_data(prefix, suffix);

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    for line in data.iter() {
        let factors: Vec<(usize, usize)> = re
            .captures_iter(line)
            .map(|caps| {
                let (_, [a, b]) = caps.extract();
                (
                    a.parse::<usize>().expect("Expect numerical value"),
                    b.parse::<usize>().expect("Expect numerical value"),
                )
            })
            .collect();
        for (a, b) in factors.iter() {
            result += a * b;
        }
    }

    result
}

enum Instruction {
    Disable,
    Enable,
    Mult(usize, usize),
}

fn solve_part_2(prefix: Option<&str>, suffix: Option<&str>) -> usize {
    let mut result: usize = 0;
    let data = load_data(prefix, suffix);

    let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))|(do\(\)|don\'t\(\))").unwrap();
    let re_mult = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in data.iter() {
        for cap in re.captures_iter(line) {
            let (_, [a]) = cap.extract();
            match a {
                "do()" => instructions.push(Instruction::Enable),
                "don't()" => instructions.push(Instruction::Disable),
                _ => {
                    let Some(cap_mult) = re_mult.captures(a) else {
                        panic!("Unknown instruction")
                    };
                    let (_, [b, c]) = cap_mult.extract();
                    instructions.push(Instruction::Mult(
                        b.parse::<usize>().expect("Expect numerical value"),
                        c.parse::<usize>().expect("Expect numerical value"),
                    ))
                }
            }
        }
    }

    let mut enabled = true;
    for instruction in instructions.iter() {
        match instruction {
            Instruction::Enable => enabled = true,
            Instruction::Disable => enabled = false,
            Instruction::Mult(a, b) => {
                if enabled {
                    result += a * b;
                }
            }
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
        assert_eq!(result, 161);
    }

    #[test]
    fn answer_part_1() {
        let result = solve_part_1(Some("../"), None);
        assert_eq!(result, 178794710);
    }

    #[test]
    fn example_part_2() {
        let result = solve_part_2(Some("../"), Some(".example2"));
        assert_eq!(result, 48);
    }

    #[test]
    fn answer_part_2() {
        let result = solve_part_2(Some("../"), None);
        assert_eq!(result, 76729637);
    }
}
