use std::collections::HashMap;
use std::fs;
use std::path::Path;

static YEAR: &str = "2024";
static DAY: &str = "24";

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
        let input1 = line_split[0];
        let input2 = line_split[2];
        gates.insert(
            name.to_string(),
            (gate_type, input1.to_string(), input2.to_string()),
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
            let input1 = if inputs.contains_key(g1) {
                *inputs.get(g1).unwrap()
            } else {
                get_gate_output(g1, inputs, gates, outputs)
            };
            let input2 = if inputs.contains_key(g2) {
                *inputs.get(g2).unwrap()
            } else {
                get_gate_output(g2, inputs, gates, outputs)
            };
            let output = match operation {
                GateType::AND => input1 && input2,
                GateType::OR => input1 || input2,
                GateType::XOR => input1 ^ input2,
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

// def get_all_gate_parents(self, name: str) -> set[str]:
//     gate = self.gates[name]
//     if not gate.input:
//         return set()

//     parents = {*gate.input}
//     for input_name in gate.input:
//         parents |= self.get_all_gate_parents(input_name)

//     return parents

// fn reset_outputs(&mut self) {
//     for gate in self.gates.values_mut() {
//         gate.output = None;
//     }
// }

// def reset_outputs(self):
//     for gate in self.gates.values():
//         if gate.input:
//             gate.output = None

// def set_random_inputs(self):
//     for gate in self.gates.values():
//         if gate.name[0] in "xy":
//             gate.output = bool(random.getrandbits(1))

// def swap_gate_outputs(self, name_1: str, name_2: str):
//     if name_1 in self.get_all_gate_parents(
//         name_2
//     ) or name_2 in self.get_all_gate_parents(name_1):
//         raise ValueError("Swap would create loop")

//     gate_1 = deepcopy(self.gates[name_1])
//     gate_2 = deepcopy(self.gates[name_2])

//     gate_1.name = name_2
//     gate_2.name = name_1

//     self.gates[name_1] = gate_2
//     self.gates[name_2] = gate_1

// @property
// def target_number(self) -> int:
//     return self.get_register_number("x") + self.get_register_number("y")

// @property
// def output_number(self) -> int:
//     return self.get_register_number("z")

//     @property
// def suspicious_gates(self) -> set[str]:
//     max_z = max(
//         int(gate.name[1:])
//         for gate in self.gates.values()
//         if gate.name.startswith("z")
//     )

//     suspicious = set()
//     # All but last output wire must be from XOR Gate
//     for k in range(max_z - 1):
//         gate = self.gates[f"z{k:02}"]
//         if gate.gate_type != "XOR":
//             # print(gate)
//             suspicious.add(gate.name)

//     # Last output wire is from an OR gate
//     gate = self.gates[f"z{max_z:02}"]
//     if gate.gate_type != "OR":
//         suspicious.add(gate.name)

//     # XOR gates take x and y wires or output z wire
//     for gate in self.gates.values():
//         if not gate.input or gate.gate_type != "XOR":
//             continue
//         if (gate.input[0][0], gate.input[1][0]) in (("x", "y"), ("y", "x")):
//             continue
//         if not gate.name.startswith("z"):
//             # print(gate)
//             suspicious.add(gate.name)

//     # XOR only takes an input bit if a XOR follows it, unless the input bits are the first bits
//     for gate in self.gates.values():
//         if not gate.input or gate.gate_type != "XOR":
//             continue
//         if (gate.input[0][0], gate.input[1][0]) not in (
//             ("x", "y"),
//             ("y", "x"),
//         ) or gate.input in (("x00", "y00"), ("y00", "x00")):
//             continue

//         connecting_gates = [
//             g for g in self.gates.values() if g.input and gate.name in g.input
//         ]
//         if len([g for g in connecting_gates if g.gate_type == "XOR"]) != 1:
//             suspicious.add(gate.name)

//     # AND gate only connect to OR gates unless inputs are x and y wires
//     for gate in self.gates.values():
//         if not gate.input or gate.gate_type != "AND":
//             continue
//         if gate.input in (("x00", "y00"), ("y00", "x00")):
//             continue
//         connecting_gates = [
//             g for g in self.gates.values() if g.input and gate.name in g.input
//         ]
//         if [g for g in connecting_gates if g.gate_type != "OR"]:
//             suspicious.add(gate.name)

//     return suspicious

fn solve_part_1(prefix: Option<&str>, suffix: Option<&str>) -> usize {
    let (inputs, gates) = load_data(prefix, suffix);
    let mut outputs: HashMap<String, bool> = HashMap::new();
    get_register("z", &inputs, &gates, &mut outputs)
}

fn solve_part_2(prefix: Option<&str>, suffix: Option<&str>) -> String {
    let data = load_data(prefix, suffix);

    // Solve part 2

    "wrong".to_string()

    //  gates = load_input(filename)
    // circuit = Circuit(gates=deepcopy(gates))

    // gate_combos = list(combinations(circuit.suspicious_gates, 2))
    // swap_sets = list(combinations(gate_combos, 4))
    // swap_sets = [
    //     swap_set
    //     for swap_set in swap_sets
    //     if len({combo for combos in swap_set for combo in combos}) == 8
    // ]

    // solutions = []
    // for swap_set in swap_sets:
    //     circuit = Circuit(gates=load_input(filename))
    //     try:
    //         for swap in swap_set:
    //             circuit.swap_gate_outputs(*swap)
    //     except ValueError:
    //         continue

    //     test_passed = True
    //     for _ in range(100):
    //         circuit.reset_outputs()
    //         circuit.set_random_inputs()
    //         if circuit.output_number != circuit.target_number:
    //             test_passed = False
    //             break

    //     if test_passed:
    //         solutions.append(swap_set)

    // answers = {
    //     ",".join(sorted([combo for swap in swap_set for combo in swap]))
    //     for swap_set in solutions
    // }
    // return "\n".join(answers)
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
