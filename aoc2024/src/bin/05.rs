use std::fs;
use std::path::Path;

static YEAR: &str = "2024";
static DAY: &str = "05";

fn load_data(prefix: Option<&str>, suffix: Option<&str>) -> (Vec<(u8, u8)>, Vec<Vec<u8>>) {
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

    let mut rules: Vec<(u8, u8)> = Vec::new();
    let mut page_lists: Vec<Vec<u8>> = Vec::new();

    for rule in parts[0].split("\n") {
        let split_rule = rule.split("|").collect::<Vec<_>>();
        rules.push((
            split_rule[0].parse::<u8>().expect("Expect numerical value"),
            split_rule[1].parse::<u8>().expect("Expect numerical value"),
        ));
    }

    for page in parts[1].split("\n") {
        if page.len() == 0 {
            continue;
        }
        page_lists.push(
            page.split(",")
                .map(|s| s.parse::<u8>().expect("Expect numerical value"))
                .collect(),
        );
    }

    (rules, page_lists)
}

fn validate_page(page: &Vec<u8>, rules: &Vec<(u8, u8)>) -> bool {
    for (first, second) in rules.iter() {
        if page.contains(first) && page.contains(second) {
            let index1 = page.iter().position(|&x| x == *first).unwrap();
            let index2 = page.iter().position(|&x| x == *second).unwrap();
            if index1 > index2 {
                return false;
            }
        }
    }
    true
}

fn fix_page(page: Vec<u8>, rules: &Vec<(u8, u8)>, max_depth: u8) -> Vec<u8> {
    if max_depth == 0 {
        return page;
    }
    let mut corrected = page.clone();
    for (first, second) in rules.iter() {
        if corrected.contains(first) && corrected.contains(second) {
            let index1 = corrected.iter().position(|&x| x == *first).unwrap();
            let index2 = corrected.iter().position(|&x| x == *second).unwrap();
            if index1 > index2 {
                corrected.remove(index1);
                corrected.insert(index2, *first);
            }
        }
    }
    if validate_page(&corrected, rules) {
        return corrected;
    }
    fix_page(corrected, rules, max_depth - 1)
}

fn solve_part_1(prefix: Option<&str>, suffix: Option<&str>) -> usize {
    let (rules, page_lists) = load_data(prefix, suffix);

    let result: usize = page_lists
        .iter()
        .filter(|&page| validate_page(page, &rules))
        .map(|page| usize::from(page[page.len() / 2]))
        .sum();

    result
}

fn solve_part_2(prefix: Option<&str>, suffix: Option<&str>) -> usize {
    let (rules, page_lists) = load_data(prefix, suffix);

    let result: usize = page_lists
        .iter()
        .filter(|&page| !validate_page(page, &rules))
        .map(|page| fix_page(page.clone(), &rules, 2))
        .map(|page| usize::from(page[page.len() / 2]))
        .sum();

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
        assert_eq!(result, 143);
    }

    #[test]
    fn answer_part_1() {
        let result = solve_part_1(Some("../"), None);
        assert_eq!(result, 4790);
    }

    #[test]
    fn example_part_2() {
        let result = solve_part_2(Some("../"), Some(".example"));
        assert_eq!(result, 123);
    }

    #[test]
    fn answer_part_2() {
        let result = solve_part_2(Some("../"), None);
        assert_eq!(result, 6319);
    }
}
