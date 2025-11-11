use std::fs;
use std::path::Path;

static YEAR: &str = "2024";
static DAY: &str = "04";

fn load_data(prefix: Option<&str>, suffix: Option<&str>) -> Vec<Vec<String>> {
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
        data.push(line.chars().map(|c| c.to_string()).collect());
    }

    data
}

fn solve_part_1(prefix: Option<&str>, suffix: Option<&str>) -> usize {
    let mut result: usize = 0;
    let data = load_data(prefix, suffix);

    let width = data[0].len();
    let height = data.len();
    for (y, line) in data.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            let mut words = Vec::new();
            if x < width - 3 {
                words.push(format!(
                    "{}{}{}{}",
                    c,
                    line[x + 1],
                    line[x + 2],
                    line[x + 3],
                ));
            }
            if y < height - 3 {
                words.push(format!(
                    "{}{}{}{}",
                    c,
                    data[y + 1][x],
                    data[y + 2][x],
                    data[y + 3][x],
                ));
            }
            if x < width - 3 && y > 2 {
                words.push(format!(
                    "{}{}{}{}",
                    c,
                    data[y - 1][x + 1],
                    data[y - 2][x + 2],
                    data[y - 3][x + 3],
                ));
            }
            if x < width - 3 && y < height - 3 {
                words.push(format!(
                    "{}{}{}{}",
                    c,
                    data[y + 1][x + 1],
                    data[y + 2][x + 2],
                    data[y + 3][x + 3],
                ));
            }
            result += words
                .iter()
                .filter(|w| *w == "XMAS" || *w == "SAMX")
                .count();
        }
    }

    result
}

fn solve_part_2(prefix: Option<&str>, suffix: Option<&str>) -> usize {
    let mut result: usize = 0;
    let data = load_data(prefix, suffix);

    let width = data[0].len();
    let height = data.len();
    for (y, line) in data.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if c != "A" || x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                continue;
            }
            let word1 = format!("{}{}{}", data[y - 1][x - 1], c, data[y + 1][x + 1],);
            let word2 = format!("{}{}{}", data[y - 1][x + 1], c, data[y + 1][x - 1],);
            if (word1 == "MAS" || word1 == "SAM") && (word2 == "MAS" || word2 == "SAM") {
                result += 1;
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
        assert_eq!(result, 18);
    }

    #[test]
    fn answer_part_1() {
        let result = solve_part_1(Some("../"), None);
        assert_eq!(result, 2536);
    }

    #[test]
    fn example_part_2() {
        let result = solve_part_2(Some("../"), Some(".example"));
        assert_eq!(result, 9);
    }

    #[test]
    fn answer_part_2() {
        let result = solve_part_2(Some("../"), None);
        assert_eq!(result, 1875);
    }
}
