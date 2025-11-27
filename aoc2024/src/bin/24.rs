use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

static YEAR: &str = "2024";
static DAY: &str = "24";

#[derive(Clone, PartialEq)]
enum GateType {
    AND,
    OR,
    XOR,
}

fn load_data(
    prefix: Option<&str>,
    suffix: Option<&str>,
) -> (
    HashMap<String, bool>,
    HashMap<String, (GateType, String, String)>,
) {
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
    let part1 = parts[0];
    let part2 = parts[1];

    let mut inputs = HashMap::new();
    for line in part1.split("\n") {
        if line.len() == 0 {
            continue;
        }
        let line_split: Vec<&str> = line.split(": ").collect();
        let name = line_split[0];
        let value = line_split[1] == "1";
        inputs.insert(name.to_string(), value);
    }

    let mut gates = HashMap::new();
    for line in part2.split("\n") {
        if line.len() == 0 {
            continue;
        }
        let line_split: Vec<&str> = line.split(" ").collect();
        let name = line_split[4];
        let gate_type = match line_split[1] {
            "AND" => GateType::AND,
            "OR" => GateType::OR,
            "XOR" => GateType::XOR,
            _ => panic!("Unknown gate type"),
        };
        let input_1 = line_split[0];
        let input_2 = line_split[2];
        gates.insert(
            name.to_string(),
            (gate_type, input_1.to_string(), input_2.to_string()),
        );
    }

    (inputs, gates)
}

fn get_gate_output(
    name: &str,
    inputs: &HashMap<String, bool>,
    gates: &HashMap<String, (GateType, String, String)>,
    outputs: &mut HashMap<String, bool>,
) -> bool {
    let (operation, g1, g2) = gates
        .get(name)
        .expect(format!("Gate not found: {name}").as_str());
    match outputs.get(name) {
        None => {
            let input_1 = if inputs.contains_key(g1) {
                *inputs.get(g1).unwrap()
            } else {
                get_gate_output(g1, inputs, gates, outputs)
            };
            let input_2 = if inputs.contains_key(g2) {
                *inputs.get(g2).unwrap()
            } else {
                get_gate_output(g2, inputs, gates, outputs)
            };
            let output = match operation {
                GateType::AND => input_1 && input_2,
                GateType::OR => input_1 || input_2,
                GateType::XOR => input_1 ^ input_2,
            };
            outputs.insert(name.to_string(), output);
            output
        }
        Some(o) => *o,
    }
}

fn get_register(
    name: &str,
    inputs: &HashMap<String, bool>,
    gates: &HashMap<String, (GateType, String, String)>,
    outputs: &mut HashMap<String, bool>,
) -> usize {
    let mut value = 0;
    for gate_name in gates.keys() {
        if gate_name.starts_with(name) && get_gate_output(gate_name, inputs, gates, outputs) {
            value += 1 << (gate_name[1..].parse::<usize>().unwrap());
        }
    }
    value
}

fn get_all_gate_parents(
    gate_name: String,
    gates: &HashMap<String, (GateType, String, String)>,
) -> HashSet<String> {
    if !gates.contains_key(&gate_name) {
        return HashSet::new();
    }
    let (_, input_1, input_2) = gates.get(&gate_name).unwrap();
    let mut parents = HashSet::from([input_1.to_string(), input_2.to_string()]);
    parents.extend(get_all_gate_parents(input_1.to_string(), gates));
    parents.extend(get_all_gate_parents(input_2.to_string(), gates));

    parents
}

fn can_swap_gates(
    gate_name_1: String,
    gate_name_2: String,
    gates: &HashMap<String, (GateType, String, String)>,
) -> bool {
    let parents_1 = get_all_gate_parents(gate_name_1.clone(), gates);
    let parents_2 = get_all_gate_parents(gate_name_2.clone(), gates);

    !parents_2.contains(&gate_name_1) && !parents_1.contains(&gate_name_2)
}

fn swap_gate_outputs(
    gate_name_1: String,
    gate_name_2: String,
    gates: &HashMap<String, (GateType, String, String)>,
) -> HashMap<String, (GateType, String, String)> {
    let mut new_gates = gates.clone();

    let gate_1 = gates.get(&gate_name_1).unwrap().clone();
    let gate_2 = gates.get(&gate_name_2).unwrap().clone();
    new_gates.insert(gate_name_1, gate_2.clone());
    new_gates.insert(gate_name_2, gate_1.clone());

    new_gates
}

// def set_random_inputs(self):
//     for gate in self.gates.values():
//         if gate.name[0] in "xy":
//             gate.output = bool(random.getrandbits(1))

fn get_suspicious_gates(gates: &HashMap<String, (GateType, String, String)>) -> HashSet<String> {
    let max_z = gates
        .keys()
        .filter(|k| k.starts_with("z"))
        .map(|k| k[1..].parse::<usize>().unwrap())
        .max()
        .unwrap();

    let mut suspicious = HashSet::new();
    // All but last output wire must be from XOR Gate
    for k in 0..(max_z - 1) {
        let gate_name = format!("z{k:02}");
        match gates.get(&gate_name) {
            None => (),
            Some((gate_type, _, _)) => {
                if *gate_type != GateType::OR {
                    suspicious.insert(gate_name);
                }
            }
        };
    }

    // Last output wire is from an OR gate
    let gate_name = format!("z{max_z:02}");
    match gates.get(&gate_name) {
        None => (),
        Some((gate_type, _, _)) => {
            if *gate_type != GateType::OR {
                suspicious.insert(gate_name);
            }
        }
    }

    for (gate_name, (gate_type, input_1, input_2)) in gates.iter() {
        if *gate_type != GateType::XOR
            || (input_1.starts_with("x") && input_2.starts_with("y"))
            || (input_1.starts_with("y") && input_2.starts_with("x"))
        {
            continue;
        }

        // XOR gates take x and y wires or output z wire
        if !gate_name.starts_with("z") {
            suspicious.insert(gate_name.to_string());
        }

        // XOR only takes an input bit if a XOR follows it, unless the input bits are the first bits
        if [("x00", "y00"), ("y00", "x00")].contains(&(input_1.as_str(), input_2.as_str())) {
            continue;
        }
        let connecting_gates = gates
            .values()
            .filter(|g| g.1 == *gate_name || g.2 == *gate_name);
        if connecting_gates.filter(|g| g.0 == GateType::XOR).count() != 1 {
            suspicious.insert(gate_name.to_string());
        }
    }

    // AND gate only connect to OR gates unless inputs are x and y wires
    for (gate_name, (gate_type, input_1, input_2)) in gates.iter() {
        if *gate_type != GateType::AND
            || [("x00", "y00"), ("y00", "x00")].contains(&(input_1.as_str(), input_2.as_str()))
        {
            continue;
        }
        let connecting_gates = gates
            .values()
            .filter(|g| g.1 == *gate_name || g.2 == *gate_name);
        if connecting_gates.filter(|g| g.0 != GateType::OR).count() > 0 {
            suspicious.insert(gate_name.to_string());
        }
    }

    suspicious
}

fn count_combinations(n: u64, r: u64) -> u64 {
    if r > n {
        0
    } else {
        (1..=r).fold(1, |acc, val| acc * (n - val + 1) / val)
    }
}

fn solve_part_1(prefix: Option<&str>, suffix: Option<&str>) -> usize {
    let (inputs, gates) = load_data(prefix, suffix);
    let mut outputs: HashMap<String, bool> = HashMap::new();
    get_register("z", &inputs, &gates, &mut outputs)
}

fn solve_part_2(prefix: Option<&str>, suffix: Option<&str>) -> String {
    let (inputs, gates) = load_data(prefix, suffix);

    let suspicious_gates = get_suspicious_gates(&gates);
    println!(
        "Found {} suspicious gates of {} gates.",
        suspicious_gates.len(),
        gates.len()
    );
    return "wrong".to_string();
    let combos = suspicious_gates.iter().combinations(2);

    let mut answers = HashSet::new();
    let bar = ProgressBar::new(count_combinations(
        combos.clone().collect::<Vec<_>>().len() as u64,
        4,
    ) as u64);
    bar.set_style(
        ProgressStyle::with_template(
            "[{elapsed}/{eta}] {bar:40.cyan/blue} {pos}/{len} {percent:3} {msg}",
        )
        .unwrap()
        .progress_chars("##-"),
    );
    for swap_set in combos.combinations(4).into_iter() {
        bar.inc(1);
        let mut unique_gates = HashSet::new();
        for swap in swap_set.clone() {
            for gate_name in swap {
                unique_gates.insert(gate_name.clone());
            }
        }
        if unique_gates.len() != 8 {
            continue;
        }

        let mut fixed_gates = gates.clone();
        let mut swap_successful = true;
        for swap in swap_set.clone() {
            if can_swap_gates(swap[0].clone(), swap[1].clone(), &fixed_gates) {
                fixed_gates = swap_gate_outputs(swap[0].clone(), swap[1].clone(), &fixed_gates);
            } else {
                swap_successful = false;
                break;
            }
        }
        if !swap_successful {
            continue;
        }

        let mut test_passed = true;
        for _ in 0..100 {
            let mut outputs: HashMap<String, bool> = HashMap::new();
            let test_inputs = inputs
                .iter()
                .map(|(k, _)| (k.to_string(), rand::random_bool(0.5)))
                .collect::<HashMap<_, _>>();
            if get_register("x", &test_inputs, &gates, &mut outputs)
                + get_register("y", &test_inputs, &gates, &mut outputs)
                != get_register("z", &test_inputs, &gates, &mut outputs)
            {
                test_passed = false;
                break;
            }
        }
        if test_passed {
            let mut gate_names = vec![];
            for swap in swap_set {
                for gate_name in swap {
                    gate_names.push(gate_name.clone());
                }
            }
            gate_names.sort();
            answers.insert(gate_names.join(","));
            break;
        }
    }
    bar.finish();

    answers.iter().join("\n")
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
