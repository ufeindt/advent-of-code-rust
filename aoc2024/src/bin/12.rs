use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

static YEAR: &str = "2024";
static DAY: &str = "12";

fn load_data(prefix: Option<&str>, suffix: Option<&str>) -> HashMap<(u8, u8), char> {
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

    let mut map = HashMap::new();
    for (y, line) in input.split("\n").enumerate() {
        if line.len() == 0 {
            continue;
        }
        for (x, c) in line.chars().enumerate() {
            map.insert(((x + 1) as u8, (y + 1) as u8), c);
        }
    }
    map
}

fn find_region(
    map: &HashMap<(u8, u8), char>,
    coord: (u8, u8),
    mut region: HashSet<(u8, u8)>,
) -> HashSet<(u8, u8)> {
    region.insert(coord);
    let (x, y) = (coord.0, coord.1);
    for coord_new in [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)] {
        if !map.contains_key(&coord_new)
            || map.get(&coord) != map.get(&coord_new)
            || region.contains(&coord_new)
        {
            continue;
        }
        let new_region = find_region(map, coord_new, region.clone());
        region.extend(new_region.iter());
    }

    region
}

fn find_all_regions(map: &HashMap<(u8, u8), char>) -> Vec<HashSet<(u8, u8)>> {
    let mut assigned_coords = HashSet::new();
    let mut regions = Vec::new();
    for coord in map.keys() {
        if assigned_coords.contains(&*coord) {
            continue;
        }
        let region = find_region(map, *coord, HashSet::new());
        assigned_coords.extend(region.clone());
        regions.push(region);
    }
    regions
}

fn get_perimeter(region: &HashSet<(u8, u8)>) -> usize {
    let mut perimeter = 0;
    for coord in region.iter() {
        let (x, y) = (coord.0, coord.1);
        for (x_new, y_new) in [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)] {
            if !region.contains(&(x_new, y_new)) {
                perimeter += 1;
            }
        }
    }

    perimeter
}

fn get_perimeter_sections(region: &HashSet<(u8, u8)>) -> usize {
    let mut fence_sections = Vec::new();
    for coord in region.iter() {
        let (x, y) = (coord.0, coord.1);
        for (k, (x_new, y_new)) in vec![(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)]
            .iter()
            .enumerate()
        {
            if !region.contains(&(*x_new, *y_new)) {
                fence_sections.push([*x_new as usize, *y_new as usize, k as usize]);
            }
        }
    }

    fence_sections.sort_by_key(|s| (s[2], s[s[2] % 2], s[(s[2] + 1) % 2]));

    let mut perimeter = 1;
    for k in 1..fence_sections.len() {
        let section_0 = fence_sections[k - 1];
        let section_1 = fence_sections[k];
        let index_eq = section_0[2] % 2;
        let index_inc = (section_0[2] + 1) % 2;
        if section_0[2] != section_1[2]
            || section_0[index_eq] != section_1[index_eq]
            || section_0[index_inc] + 1 != section_1[index_inc]
        {
            perimeter += 1;
        }
    }

    perimeter
}

fn solve_part_1(prefix: Option<&str>, suffix: Option<&str>) -> usize {
    let map = load_data(prefix, suffix);
    let regions = find_all_regions(&map);
    regions
        .iter()
        .map(|region| get_perimeter(&region) * region.len())
        .sum()
}

fn solve_part_2(prefix: Option<&str>, suffix: Option<&str>) -> usize {
    let map = load_data(prefix, suffix);
    let regions = find_all_regions(&map);
    regions
        .iter()
        .map(|region| get_perimeter_sections(&region) * region.len())
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

    #[test]
    fn example_part_1() {
        let result = solve_part_1(Some("../"), Some(".example"));
        assert_eq!(result, 1930);
    }

    #[test]
    fn answer_part_1() {
        let result = solve_part_1(Some("../"), None);
        assert_eq!(result, 1533024);
    }

    #[test]
    fn example_part_2() {
        let result = solve_part_2(Some("../"), Some(".example"));
        assert_eq!(result, 1206);
    }

    #[test]
    fn answer_part_2() {
        let result = solve_part_2(Some("../"), None);
        assert_eq!(result, 910066);
    }
}
