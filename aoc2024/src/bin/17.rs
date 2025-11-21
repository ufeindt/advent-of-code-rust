use std::fs;
use std::path::Path;

static YEAR: &str = "2024";
static DAY: &str = "17";

struct Computer {
    a: isize,
    b: isize,
    c: isize,
    program: Vec<isize>,
    pointer: usize,
    ouput: Vec<isize>,
}

impl Computer {
    fn new(program: Vec<isize>, a: isize) -> Computer {
        Computer {
            a,
            b: 0,
            c: 0,
            program,
            pointer: 0,
            ouput: Vec::new(),
        }
    }

    fn get_combo_operands(&self, operand: isize) -> isize {
        if operand <= 3 {
            return operand;
        } else if operand == 4 {
            return self.a;
        } else if operand == 5 {
            return self.b;
        } else if operand == 6 {
            return self.c;
        }
        panic!("Invalid operand: {}", operand)
    }

    fn run(&mut self) {
        loop {
            if self.pointer >= self.program.len() - 1 {
                break;
            }

            let (op_code, operand) = (self.program[self.pointer], self.program[self.pointer + 1]);
            match op_code {
                0 => {
                    self.a >>= self.get_combo_operands(operand);
                }
                1 => {
                    self.b ^= operand;
                }
                2 => {
                    self.b = self.get_combo_operands(operand) % 8;
                }
                3 => {
                    if self.a != 0 {
                        self.pointer = operand as usize;
                        continue;
                    }
                }
                4 => {
                    self.b ^= self.c;
                }
                5 => {
                    self.ouput.push(self.get_combo_operands(operand) % 8);
                }
                6 => {
                    self.b = self.a >> self.get_combo_operands(operand);
                }
                7 => {
                    self.c = self.a >> self.get_combo_operands(operand);
                }
                _ => (),
            }
            self.pointer += 2;
        }
    }
}

fn load_data(prefix: Option<&str>, suffix: Option<&str>) -> Computer {
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

    let lines = input.split("\n").collect::<Vec<_>>();
    let register_a = lines[0].split(": ").collect::<Vec<_>>()[1]
        .parse::<isize>()
        .unwrap();
    let program = lines[4].split(": ").collect::<Vec<_>>()[1]
        .split(",")
        .map(|s| s.parse::<isize>().unwrap())
        .collect();

    Computer::new(program, register_a)
}

fn solve_part_1(prefix: Option<&str>, suffix: Option<&str>) -> String {
    let mut computer = load_data(prefix, suffix);
    computer.run();
    computer
        .ouput
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn solve_part_2(prefix: Option<&str>, suffix: Option<&str>) -> isize {
    let program = load_data(prefix, suffix).program;
    let mut solutions = vec![0];
    let mut targets = vec![];
    for k in 1..(program.len() + 1) {
        let mut target = vec![];
        for l in (program.len() - k)..program.len() {
            target.push(program[l]);
        }
        targets.push(target);
    }

    for target in targets {
        let mut new_solutions = vec![];
        for solution in solutions {
            for a_ in 0..8 {
                let mut computer = Computer::new(program.clone(), 8 * solution + a_);
                computer.run();
                if computer.ouput == target {
                    new_solutions.push(8 * solution + a_);
                }
            }
        }
        solutions = new_solutions;
    }

    *solutions.iter().min().unwrap()
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
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn answer_part_1() {
        let result = solve_part_1(Some("../"), None);
        assert_eq!(result, "1,3,7,4,6,4,2,3,5");
    }

    #[test]
    fn example_part_2() {
        let result = solve_part_2(Some("../"), Some(".example2"));
        assert_eq!(result, 117440);
    }

    #[test]
    fn answer_part_2() {
        let result = solve_part_2(Some("../"), None);
        assert_eq!(result, 202367025818154);
    }
}
