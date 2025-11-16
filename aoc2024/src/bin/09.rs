use std::fs;
use std::path::Path;

static YEAR: &str = "2024";
static DAY: &str = "09";

#[derive(Clone, Copy, Debug, PartialEq)]
enum BlockType {
    Empty,
    File,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Block {
    number: Option<usize>,
    size: usize,
    block_type: BlockType,
}

impl Block {
    fn new_empty(size: usize) -> Block {
        Block {
            number: None,
            size: size,
            block_type: BlockType::Empty,
        }
    }

    fn new_file(number: usize, size: usize) -> Block {
        Block {
            number: Some(number),
            size: size,
            block_type: BlockType::File,
        }
    }
}

fn load_data(prefix: Option<&str>, suffix: Option<&str>) -> Vec<Block> {
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

    let mut blocks = Vec::new();
    for (k, char) in input.split("\n").next().unwrap().chars().enumerate() {
        let size = char.to_digit(10).unwrap() as usize;
        if k % 2 == 0 {
            blocks.push(Block::new_file(k / 2, size));
        } else if size > 0 {
            blocks.push(Block::new_empty(size));
        }
    }

    blocks
}

fn get_checksum(blocks: Vec<Block>) -> usize {
    let mut checksum: usize = 0;

    let mut k: usize = 0;
    for block in blocks {
        match block.block_type {
            BlockType::Empty => (),
            BlockType::File => {
                let number = block.number.unwrap();
                for l in k..k + block.size {
                    checksum += number * l;
                }
            }
        }
        k += block.size;
    }

    checksum
}

fn solve_part_1(prefix: Option<&str>, suffix: Option<&str>) -> usize {
    let mut blocks = load_data(prefix, suffix);

    loop {
        let mut last_block = blocks.pop().unwrap();
        if last_block.block_type == BlockType::Empty {
            continue;
        }
        let mut finished = false;
        loop {
            match blocks.iter().position(|b| b.block_type == BlockType::Empty) {
                None => {
                    blocks.push(last_block);
                    finished = true;
                    break;
                }
                Some(i) => {
                    let first_empty_index = i;
                    let first_empty_size = blocks[first_empty_index].size;
                    blocks.remove(first_empty_index);
                    if first_empty_size >= last_block.size {
                        blocks.insert(first_empty_index, last_block);
                        if first_empty_size > last_block.size {
                            blocks.insert(
                                first_empty_index + 1,
                                Block::new_empty(first_empty_size - last_block.size),
                            );
                        }
                        break;
                    } else {
                        blocks.insert(
                            first_empty_index,
                            Block::new_file(last_block.number.unwrap(), first_empty_size),
                        );
                        last_block.size -= first_empty_size;
                    }
                }
            }
        }
        if finished {
            break;
        }
    }

    get_checksum(blocks)
}

fn solve_part_2(prefix: Option<&str>, suffix: Option<&str>) -> usize {
    let mut blocks = load_data(prefix, suffix);

    let max_number = blocks
        .iter()
        .rev()
        .filter(|b| b.block_type == BlockType::File)
        .next()
        .unwrap()
        .number
        .unwrap();

    for number in (1..max_number + 1).rev() {
        let block_index = blocks
            .iter()
            .position(|b| b.block_type == BlockType::File && b.number.unwrap() == number)
            .unwrap();
        let block_size = blocks[block_index].size;
        let mut empty_block_index_and_size: Option<(usize, usize)> = None;
        for (k, empty_block) in blocks.iter().enumerate().skip(1) {
            if k == block_index {
                break;
            }
            if empty_block.block_type == BlockType::File {
                continue;
            }
            if empty_block.size >= block_size {
                empty_block_index_and_size = Some((k, empty_block.size));
                break;
            }
        }
        match empty_block_index_and_size {
            None => (),
            Some((k, empty_block_size)) => {
                blocks.remove(k);
                blocks.insert(k, Block::new_file(number, block_size));
                blocks.remove(block_index);
                if block_index >= blocks.len() {
                    blocks.push(Block::new_empty(block_size));
                } else {
                    if blocks[block_index].block_type == BlockType::File {
                        blocks.insert(block_index, Block::new_empty(block_size));
                    } else {
                        blocks[block_index].size += block_size;
                    }
                }
                if empty_block_size > block_size {
                    if blocks[k + 1].block_type == BlockType::File {
                        blocks.insert(k + 1, Block::new_empty(empty_block_size - block_size));
                    } else {
                        blocks[k + 1].size += empty_block_size - block_size;
                    }
                }
            }
        }
    }

    get_checksum(blocks)
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
        assert_eq!(result, 1928);
    }

    #[test]
    fn answer_part_1() {
        let result = solve_part_1(Some("../"), None);
        assert_eq!(result, 6200294120911);
    }

    #[test]
    fn example_part_2() {
        let result = solve_part_2(Some("../"), Some(".example"));
        assert_eq!(result, 2858);
    }

    #[test]
    fn answer_part_2() {
        let result = solve_part_2(Some("../"), None);
        assert_eq!(result, 6227018762750);
    }
}
