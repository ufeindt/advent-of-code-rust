use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::Hash;
use std::path::Path;

static YEAR: &str = "2024";
static DAY: &str = "06";

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Node {
    x: u8,
    y: u8,
    facing: Direction,
}

enum NextNode {
    Moved(Node),
    Turned(Node),
    Exited,
}

struct NodeMap {
    width: u8,
    height: u8,
    next_nodes: HashMap<Node, NextNode>,
}

impl NodeMap {
    fn get_next_node(&self, node: &Node) -> Option<&NextNode> {
        self.next_nodes.get(node)
    }

    fn new(width: u8, height: u8) -> NodeMap {
        let mut next_nodes = HashMap::new();
        for x in 0..width {
            for y in 0..height {
                next_nodes.insert(
                    Node {
                        x,
                        y,
                        facing: Direction::Right,
                    },
                    if x < width - 1 {
                        NextNode::Moved(Node {
                            x: x + 1,
                            y,
                            facing: Direction::Right,
                        })
                    } else {
                        NextNode::Exited
                    },
                );
                next_nodes.insert(
                    Node {
                        x,
                        y,
                        facing: Direction::Down,
                    },
                    if y < height - 1 {
                        NextNode::Moved(Node {
                            x,
                            y: y + 1,
                            facing: Direction::Down,
                        })
                    } else {
                        NextNode::Exited
                    },
                );
                next_nodes.insert(
                    Node {
                        x,
                        y,
                        facing: Direction::Left,
                    },
                    if x > 0 {
                        NextNode::Moved(Node {
                            x: x - 1,
                            y,
                            facing: Direction::Left,
                        })
                    } else {
                        NextNode::Exited
                    },
                );
                next_nodes.insert(
                    Node {
                        x,
                        y,
                        facing: Direction::Up,
                    },
                    if y > 0 {
                        NextNode::Moved(Node {
                            x,
                            y: y - 1,
                            facing: Direction::Up,
                        })
                    } else {
                        NextNode::Exited
                    },
                );
            }
        }
        NodeMap {
            width,
            height,
            next_nodes,
        }
    }

    fn add_obstacle(&mut self, x: u8, y: u8) {
        if x > 0 {
            self.next_nodes.insert(
                Node {
                    x: x - 1,
                    y,
                    facing: Direction::Right,
                },
                NextNode::Turned(Node {
                    x: x - 1,
                    y,
                    facing: Direction::Down,
                }),
            );
        }
        if y > 0 {
            self.next_nodes.insert(
                Node {
                    x,
                    y: y - 1,
                    facing: Direction::Down,
                },
                NextNode::Turned(Node {
                    x,
                    y: y - 1,
                    facing: Direction::Left,
                }),
            );
        }
        if x < self.width - 1 {
            self.next_nodes.insert(
                Node {
                    x: x + 1,
                    y,
                    facing: Direction::Left,
                },
                NextNode::Turned(Node {
                    x: x + 1,
                    y,
                    facing: Direction::Up,
                }),
            );
        }
        if y < self.height - 1 {
            self.next_nodes.insert(
                Node {
                    x,
                    y: y + 1,
                    facing: Direction::Up,
                },
                NextNode::Turned(Node {
                    x,
                    y: y + 1,
                    facing: Direction::Right,
                }),
            );
        }
    }

    fn remove_obstacle(&mut self, x: u8, y: u8) {
        if x > 0 {
            self.next_nodes.insert(
                Node {
                    x: x - 1,
                    y,
                    facing: Direction::Right,
                },
                NextNode::Moved(Node {
                    x,
                    y,
                    facing: Direction::Right,
                }),
            );
        }
        if y > 0 {
            self.next_nodes.insert(
                Node {
                    x,
                    y: y - 1,
                    facing: Direction::Down,
                },
                NextNode::Moved(Node {
                    x,
                    y,
                    facing: Direction::Down,
                }),
            );
        }
        if x < self.width - 1 {
            self.next_nodes.insert(
                Node {
                    x: x + 1,
                    y,
                    facing: Direction::Left,
                },
                NextNode::Moved(Node {
                    x,
                    y,
                    facing: Direction::Left,
                }),
            );
        }
        if y < self.height - 1 {
            self.next_nodes.insert(
                Node {
                    x,
                    y: y + 1,
                    facing: Direction::Up,
                },
                NextNode::Moved(Node {
                    x,
                    y,
                    facing: Direction::Up,
                }),
            );
        }
    }
}

enum PathResult {
    Exited(Vec<Node>),
    Loop,
}

fn load_data(prefix: Option<&str>, suffix: Option<&str>) -> (NodeMap, Node) {
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

    let width = input.split("\n").next().unwrap().len() as u8;
    let height = input.split("\n").filter(|l| l.len() > 0).count() as u8;
    let mut node_map = NodeMap::new(width, height);
    let mut starting_node = Node {
        x: 0,
        y: 0,
        facing: Direction::Up,
    };
    for (y, line) in input.split("\n").enumerate() {
        if line.len() == 0 {
            continue;
        }
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {}
                '#' => {
                    node_map.add_obstacle(x as u8, y as u8);
                }
                _ => {
                    starting_node = Node {
                        x: x as u8,
                        y: y as u8,
                        facing: match c {
                            '^' => Direction::Up,
                            'v' => Direction::Down,
                            '<' => Direction::Left,
                            '>' => Direction::Right,
                            _ => panic!("Unexpected character"),
                        },
                    };
                }
            }
        }
    }

    (node_map, starting_node)
}

fn get_guard_path(map: &NodeMap, mut path: Vec<Node>) -> PathResult {
    let mut node = path.last().expect("Path must not be empty").clone();
    let mut previous_nodes = HashSet::new();
    previous_nodes.insert(node.clone());
    loop {
        match map.get_next_node(&node).expect("No next node found.") {
            NextNode::Exited => return PathResult::Exited(path),
            NextNode::Moved(n) => node = n.clone(),
            NextNode::Turned(n) => node = n.clone(),
        }
        if previous_nodes.contains(&node) {
            return PathResult::Loop;
        }
        path.push(node.clone());
        previous_nodes.insert(node.clone());
    }
}

fn get_guard_visited(map: &NodeMap, node: Node) -> HashSet<(u8, u8)> {
    match get_guard_path(map, vec![node]) {
        PathResult::Exited(path) => path
            .into_iter()
            .map(|n| (n.x, n.y))
            .collect::<HashSet<(u8, u8)>>(),
        _ => panic!("Guard should have exited"),
    }
}

fn solve_part_1(prefix: Option<&str>, suffix: Option<&str>) -> usize {
    let (map, node) = load_data(prefix, suffix);
    let visited = get_guard_visited(&map, node.clone());

    visited.len()
}

fn solve_part_2(prefix: Option<&str>, suffix: Option<&str>) -> usize {
    let mut result: usize = 0;
    let (mut map, node) = load_data(prefix, suffix);
    let original_path = match get_guard_path(&map, vec![node.clone()]) {
        PathResult::Exited(path) => path,
        _ => panic!("Guard should have exited"),
    };

    let mut tested = HashSet::new();
    tested.insert((node.x, node.y));
    let mut path = vec![node.clone()];
    for node in original_path.iter().skip(1) {
        if tested.contains(&(node.x, node.y)) {
            continue;
        }
        map.add_obstacle(node.x, node.y);
        match get_guard_path(&map, path.clone()) {
            PathResult::Loop => result += 1,
            _ => {}
        }
        map.remove_obstacle(node.x, node.y);
        path.push(node.clone());
        tested.insert((node.x, node.y));
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
        assert_eq!(result, 41);
    }

    #[test]
    fn answer_part_1() {
        let result = solve_part_1(Some("../"), None);
        assert_eq!(result, 5208);
    }

    #[test]
    fn example_part_2() {
        let result = solve_part_2(Some("../"), Some(".example"));
        assert_eq!(result, 6);
    }

    #[test]
    fn answer_part_2() {
        let result = solve_part_2(Some("../"), None);
        assert_eq!(result, 1972);
    }
}
