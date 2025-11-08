// use std::env;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

static YEAR: &str = "2024";
static DAY: &str = "01";

fn load_data(example_data: bool, test: bool) -> (Vec<usize>, Vec<usize>) {
    let mut file_name = if example_data {
        format!("input/{YEAR}/{DAY}.example.input")
    } else {
        format!("input/{YEAR}/{DAY}.input")
    };
    if test {
        file_name = format!("../{file_name}");
    }

    let input =
        fs::read_to_string(Path::new(&file_name)).expect("Should have been able to read the file");

    let mut vec1: Vec<usize> = Vec::new();
    let mut vec2: Vec<usize> = Vec::new();

    for line in input.split("\n") {
        if line.len() > 0 {
            let split_line = line.split("   ").collect::<Vec<_>>();
            if split_line.len() == 2 {
                vec1.push(
                    split_line[0]
                        .parse::<usize>()
                        .expect("Expect numerical value"),
                );
                vec2.push(
                    split_line[1]
                        .parse::<usize>()
                        .expect("Expect numerical value"),
                );
            }
        }
    }

    (vec1, vec2)
}

fn solve_part_1(example_data: bool, test: bool) -> usize {
    let (mut vec1, mut vec2) = load_data(example_data, test);
    vec1.sort();
    vec2.sort();

    let mut result: usize = 0;
    for (val1, val2) in vec1.iter().zip(vec2.iter()) {
        if val1 > val2 {
            result += val1 - val2
        } else {
            result += val2 - val1
        }
    }

    result
}

fn solve_part_2(example_data: bool, test: bool) -> usize {
    let (vec1, vec2) = load_data(example_data, test);

    let mut count = HashMap::new();
    for val in vec2.iter() {
        *count.entry(val).or_insert(0) += 1;
    }

    let mut result: usize = 0;
    for val in vec1.iter() {
        result += val * count.get(val).unwrap_or(&0);
    }
    result
}

fn main() {
    let answer1 = solve_part_1(false, false);
    println!("Answer for part 1: {answer1}");

    let answer2 = solve_part_2(false, false);
    println!("Answer for part 2: {answer2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        let result = solve_part_1(true, true);
        assert_eq!(result, 11);
    }

    #[test]
    fn answer_part_1() {
        let result = solve_part_1(false, true);
        assert_eq!(result, 2970687);
    }

    #[test]
    fn example_part_2() {
        let result = solve_part_2(true, true);
        assert_eq!(result, 31);
    }

    #[test]
    fn answer_part_2() {
        let result = solve_part_2(false, true);
        assert_eq!(result, 23963899);
    }
}
