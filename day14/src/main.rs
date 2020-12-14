use regex::Regex;
use std::collections::HashMap;

fn parse_mask_line(line: &str) -> &str {
    let re = Regex::new(r"mask = ([10X]+)").unwrap();
    return re.captures(line).unwrap().get(1).unwrap().as_str();
}

fn parse_mem_line(line: &str) -> (u64, u64) {
    let re = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
    let captures = re.captures(line).unwrap();
    return (
        captures
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .unwrap_or_default(),
        captures
            .get(2)
            .unwrap()
            .as_str()
            .parse()
            .unwrap_or_default(),
    );
}

fn set_bit_0(value: u64, bit_pos: usize) -> u64 {
    value & !(1 << bit_pos)
}

fn set_bit_1(value: u64, bit_pos: usize) -> u64 {
    value | (1 << bit_pos)
}

fn check_bit(value: u64, bit_pos: usize) -> bool {
    return value & (1 << bit_pos) != 0;
}

fn part1(input: &str) -> u64 {
    let mut mask = "";
    let mut mem = HashMap::new();
    for line in input.lines() {
        match &line[..3] {
            "mas" => mask = parse_mask_line(line),
            "mem" => {
                let (addr, mut value) = parse_mem_line(line);
                value = mask
                    .chars()
                    .rev()
                    .enumerate()
                    .fold(value, |v, (i, c)| match c {
                        '0' => set_bit_0(v, i),
                        '1' => set_bit_1(v, i),
                        'X' => v,
                        _ => panic!("Unrecognized char in mask {}", c),
                    });
                mem.insert(addr, value);
            }

            _ => panic!("Unrecognized line format"),
        }
    }
    return mem.values().sum();
}

fn part2(input: &str) -> u64 {
    let mut mask = "";
    let mut mem = HashMap::new();
    for line in input.lines() {
        match &line[..3] {
            "mas" => mask = parse_mask_line(line),
            "mem" => {
                let (mut addr, value) = parse_mem_line(line);
                //dbg!(value);
                let mut x_s = Vec::new();
                mask.chars().rev().enumerate().for_each(|(i, c)| match c {
                    '0' => return,
                    '1' => addr = set_bit_1(addr, i),
                    'X' => x_s.push(i),
                    _ => panic!("Unrecognized char in mask {}", c),
                });

                let orig_addr = addr;
                for x in 0..2u64.pow(x_s.len() as u32) {
                    let addr = x_s.iter().enumerate().fold(orig_addr, |v, (i,b)| {
                       match check_bit(x, i) {
                          true =>  set_bit_1(v, *b),
                          false =>  set_bit_0(v, *b)
                        }
                    });
                    //dbg!(addr);
                    mem.insert(addr, value);
                }
            }
            _ => panic!("Unrecognized line format"),
        }
    }

    return mem.values().sum();
}

fn main() {
    let raw_input = std::fs::read_to_string("src/input.txt").expect("Error reading the file!");
    println!("Part1 {}", part1(raw_input.as_str().clone()));
    println!("Part2 {}", part2(raw_input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

        assert_eq!(part1(input), 165);
    }

    #[test]
    fn test_part2() {
        let input = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

        assert_eq!(part2(input), 208);
    }
}
