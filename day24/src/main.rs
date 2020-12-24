use itertools::Itertools;
use std::collections::HashMap;

fn parse_line(line: &str) -> Vec<&str> {
    let mut directions = vec![];
    let mut chars = line.chars();
    while let Some(c) = chars.next() {
        let dir = match c {
            'w' => "w",
            'e' => "e",
            _ => {
                let c1 = chars.next().unwrap();
                match (c, c1) {
                    ('n', 'w') => "nw",
                    ('n', 'e') => "ne",
                    ('s', 'e') => "se",
                    ('s', 'w') => "sw",
                    _ => panic!(),
                }
            }
        };
        directions.push(dir);
    }
    directions
}

fn get_directions() -> HashMap<&'static str, (i32, i32)>{
  maplit::hashmap! {
    "e" => (1, 0),
    "se" => (0, -1),
    "sw" => (-1, -1),
    "w" =>  (-1, 0),
    "nw" => (0, 1),
    "ne" => (1, 1)  }
}

fn part1(input: &str) -> u32 {
    let instructions: Vec<Vec<&str>> = input.lines().map(|l| parse_line(l)).collect();
    let mut floor: HashMap<(i32, i32), bool> = HashMap::new();
    let directions_map = get_directions();
    instructions.iter().for_each(|inst| {
        let mut pos = (0, 0);
        inst.iter().for_each(|i| {
            let d = directions_map[i];
            pos.0 += d.0;
            pos.1 += d.1;
        });
        floor.entry(pos).and_modify(|v| *v = !*v).or_insert(true);
    });
    floor.values().fold(0, |acc, v| acc + (*v as u32))
}

fn part2(input: &str) -> u32 {
    let directions_map = get_directions();

    let instructions: Vec<Vec<&str>> = input.lines().map(|l| parse_line(l)).collect();
    let mut floor: HashMap<(i32, i32), bool> = HashMap::new();
    instructions.iter().for_each(|inst| {
        let mut pos = (0, 0);
        inst.iter().for_each(|i| {
            let d = directions_map[i];
            pos.0 += d.0;
            pos.1 += d.1;
        });
        floor.entry(pos).and_modify(|v| *v = !*v).or_insert(true);
    });

    for _ in 0..100 {
        adjust_floor_grid(&mut floor);
        let mut to_flip: Vec<(i32, i32)> = vec![];
        floor.keys().for_each(|k| {
            if should_flip(*k, &floor) {
                to_flip.push(*k);
            }
        });
        to_flip.iter().for_each(|k| {
            floor.entry(*k).and_modify(|v| *v = !*v);
        });
    }

    floor.values().fold(0, |acc, v| acc + (*v as u32))
}

fn should_flip(pos: (i32, i32), floor: &HashMap<(i32, i32), bool>) -> bool {
    let blacks = get_directions().values()
        .fold(0, |acc, v| match floor.get(&(pos.0 + v.0, pos.1 + v.1)) {
            Some(c) => {
                if *c {
                    acc + 1
                } else {
                    acc
                }
            }
            None => acc,
        });
    match floor[&pos] {
        false => blacks == 2,
        true => blacks == 0 || blacks > 2,
    }
}

fn adjust_floor_grid(floor: &mut HashMap<(i32, i32), bool>) {
    let max_x = floor.keys().map(|(x, _)| x).max().unwrap();
    let min_x = floor.keys().map(|(x, _)| x).min().unwrap();
    let max_y = floor.keys().map(|(_, y)| y).max().unwrap();
    let min_y = floor.keys().map(|(_, y)| y).min().unwrap();

    (min_x - 2..=max_x + 2)
        .cartesian_product(min_y - 2..=max_y + 2)
        .for_each(|(x, y)| {
            floor.entry((x, y)).and_modify(|_| ()).or_insert(false);
        });
}

fn main() {
    let raw_input = std::fs::read_to_string("src/input.txt").expect("Error reading the file!");
    println!("Part1: {}", part1(&raw_input));
    println!("Part2: {}", part2(&raw_input));
}
