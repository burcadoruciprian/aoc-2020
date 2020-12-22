use itertools::Itertools;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashSet, VecDeque};
use std::hash::{Hash, Hasher};

fn parse_input(input: &str) -> (VecDeque<u32>, VecDeque<u32>) {
    let (deck1, deck2) = input.split("\n\n").collect_tuple().unwrap();
    (
        deck1
            .lines()
            .filter_map(|v| v.parse::<u32>().ok())
            .collect::<VecDeque<u32>>(),
        deck2
            .lines()
            .filter_map(|v| v.parse::<u32>().ok())
            .collect::<VecDeque<u32>>(),
    )
}

fn part1(p1: VecDeque<u32>, p2: VecDeque<u32>) -> u32 {
    let mut p1 = p1.clone();
    let mut p2 = p2.clone();
    loop {
        match (p1.is_empty(), p2.is_empty()) {
            (true, false) => return get_score(&p2),
            (false, true) => return get_score(&p1),
            (false, false) => {}
            (true, true) => panic!(),
        };

        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();
        if c1 > c2 {
            p1.extend([c1, c2].iter());
        } else {
            p2.extend([c2, c1].iter());
        }
    }
}

fn part2(p1: VecDeque<u32>, p2: VecDeque<u32>) -> u32 {
    let (_, winner) = recursive_combat(p1, p2);
    return get_score(&winner);
}

fn recursive_combat(mut p1: VecDeque<u32>, mut p2:VecDeque<u32>) -> (u32, VecDeque<u32>) {
    let mut history = HashSet::new();
    loop {
        let game_hash = get_game_hash(&p1, &p2);
        if !history.insert(game_hash) {
            return (1, p1.clone()); //History repeats player 1 wins
        }

        match (p1.is_empty(), p2.is_empty()) {
            (true, false) => return (2, p2.clone()),
            (false, true) => return (1, p1.clone()),
            (false, false) => {}
            (true, true) => panic!(),
        };

        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();

        let winner = if p1.len() as u32 >= c1 && p2.len() as u32 >= c2 {
          recursive_combat(p1.iter().take(c1 as usize).cloned().collect(), p2.iter().take(c2 as usize).cloned().collect()).0
        } else {
            match c1 < c2 {
                true => 2,
                false => 1,
            }
        };

        match winner {
            1 => p1.extend([c1, c2].iter()),
            2 => p2.extend([c2, c1].iter()),
            _ => panic!(),
        }
    }
}

fn get_game_hash(p1: &VecDeque<u32>, p2: &VecDeque<u32>) -> u64 {
    let mut hasher = DefaultHasher::new();
    p1.hash(&mut hasher);
    p2.hash(&mut hasher);
    hasher.finish()
}

fn get_score(p: &VecDeque<u32>) -> u32 {
    return p
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, v)| acc + (i + 1) as u32 * v);
}

fn main() {
    let raw_input = std::fs::read_to_string("src/input.txt").expect("Error reading the file!");
    let (p1, p2) = parse_input(raw_input.as_str());
    println!("Part1 {}", part1(p1.clone(), p2.clone()));
    println!("Part2 {}", part2(p1, p2));
}
