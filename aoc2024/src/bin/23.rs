use std::collections::HashSet;
use std::fs;
use std::path::Path;

static YEAR: &str = "2024";
static DAY: &str = "23";

fn load_data(prefix: Option<&str>, suffix: Option<&str>) -> Vec<(String, String)> {
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
        let line_split: Vec<&str> = line.split("-").collect();
        data.push((line_split[0].to_string(), line_split[1].to_string()));
    }

    data
}

fn group_connections(
    connection_list: Vec<(String, String)>,
) -> std::collections::HashMap<String, std::collections::HashSet<String>> {
    let mut connections = std::collections::HashMap::new();
    for (c1, c2) in connection_list {
        if !connections.contains_key(&c1) {
            connections.insert(c1.clone(), std::collections::HashSet::new());
        }
        connections.get_mut(&c1).unwrap().insert(c2.clone());
        if !connections.contains_key(&c2) {
            connections.insert(c2.clone(), std::collections::HashSet::new());
        }
        connections.get_mut(&c2).unwrap().insert(c1.clone());
    }
    connections
}

fn find_maximum_clique(
    connections: &std::collections::HashMap<String, std::collections::HashSet<String>>,
    vertex: String,
) -> std::collections::HashSet<String> {
    let max_len = connections.values().map(|e| e.len()).max().unwrap() + 1;
    let mut clique = std::collections::HashSet::from([vertex.clone()]);

    for (check_vertex, edges) in connections.iter() {
        let check_clique = edges & &clique;
        if check_clique == clique {
            clique.insert(check_vertex.clone());
        }
        if clique.len() == max_len {
            break;
        }
    }

    clique
}

fn solve_part_1(prefix: Option<&str>, suffix: Option<&str>) -> usize {
    let data = load_data(prefix, suffix);
    let connections = group_connections(data);

    let mut groups = HashSet::new();
    for (c1, connection_set) in connections.iter() {
        if !c1.starts_with("t") {
            continue;
        }

        for c2 in connection_set {
            for c3 in connection_set {
                if c2 == c3 {
                    continue;
                }
                if connections[c2].contains(c3) {
                    let mut group = vec![c1, c2, c3];
                    group.sort();
                    groups.insert(group);
                }
            }
        }
    }
    groups.len()
}

fn solve_part_2(prefix: Option<&str>, suffix: Option<&str>) -> String {
    let data = load_data(prefix, suffix);
    let connections = group_connections(data);

    let mut biggest_clique = HashSet::new();
    for vertex in connections.keys() {
        let clique = find_maximum_clique(&connections, vertex.clone());
        if clique.len() > biggest_clique.len() {
            biggest_clique = clique;
        }
    }

    let mut result = biggest_clique.iter().map(|e| e.clone()).collect::<Vec<_>>();
    result.sort();
    result.join(",").to_string()
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
        assert_eq!(result, 7);
    }

    #[test]
    fn answer_part_1() {
        let result = solve_part_1(Some("../"), None);
        assert_eq!(result, 1151);
    }

    #[test]
    fn example_part_2() {
        let result = solve_part_2(Some("../"), Some(".example"));
        assert_eq!(result, "co,de,ka,ta");
    }

    #[test]
    fn answer_part_2() {
        let result = solve_part_2(Some("../"), None);
        assert_eq!(result, "ar,cd,hl,iw,jm,ku,qo,rz,vo,xe,xm,xv,ys");
    }
}
