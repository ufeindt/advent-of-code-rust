use std::collections::HashMap;
use std::fs;
use std::path::Path;

static YEAR: &str = "2024";
static DAY: &str = "10";

fn load_data(prefix: Option<&str>, suffix: Option<&str>) -> Vec<Vec<u8>> {
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

    let mut map = Vec::new();
    for line in input.split("\n") {
        if line.len() == 0 {
            continue;
        }
        map.push(
            line.chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect(),
        );
    }
    map
}

fn get_trailheads(map: &Vec<Vec<u8>>) -> Vec<(u8, u8)> {
    let mut trailheads = Vec::new();
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 0 {
                trailheads.push((x as u8, y as u8));
            }
        }
    }
    trailheads
}

fn score_peaks(map: &Vec<Vec<u8>>, start: (u8, u8), elevation: u8) -> HashMap<(u8, u8), u32> {
    let mut peaks = HashMap::new();
    if elevation == 9 {
        peaks.insert(start, 1);
        return peaks;
    }

    let (x, y) = (start.0 as i8, start.1 as i8);
    for (x_new, y_new) in [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)] {
        if x_new == -1 || x_new == map[0].len() as i8 || y_new == -1 || y_new == map.len() as i8 {
            continue;
        }
        if map[y_new as usize][x_new as usize] == elevation + 1 {
            for (peak, score) in score_peaks(map, (x_new as u8, y_new as u8), elevation + 1).iter()
            {
                if let Some(s) = peaks.get_mut(peak) {
                    *s += *score;
                } else {
                    peaks.insert(*peak, *score);
                }
            }
        }
    }

    peaks
}

fn solve_part_1(prefix: Option<&str>, suffix: Option<&str>) -> usize {
    let map = load_data(prefix, suffix);
    let trailheads = get_trailheads(&map);

    trailheads
        .iter()
        .map(|trailhead| score_peaks(&map, *trailhead, 0).len())
        .sum::<usize>()
}

fn solve_part_2(prefix: Option<&str>, suffix: Option<&str>) -> u32 {
    let map = load_data(prefix, suffix);
    let trailheads = get_trailheads(&map);

    trailheads
        .iter()
        .map(|trailhead| score_peaks(&map, *trailhead, 0).values().sum::<u32>())
        .sum::<u32>()
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
        assert_eq!(result, 36);
    }

    #[test]
    fn answer_part_1() {
        let result = solve_part_1(Some("../"), None);
        assert_eq!(result, 531);
    }

    #[test]
    fn example_part_2() {
        let result = solve_part_2(Some("../"), Some(".example"));
        assert_eq!(result, 81);
    }

    #[test]
    fn answer_part_2() {
        let result = solve_part_2(Some("../"), None);
        assert_eq!(result, 1210);
    }
}
