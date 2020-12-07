use std::collections::HashSet;
fn main() {
    let raw_input = std::fs::read_to_string("src/input.txt").expect("Error reading the file!");

    let groups: Vec<String> = raw_input
        .split("\n\n")
        .map(|g| g.chars().collect())
        .collect();

    println!(
        "Part1: {}",
        groups
            .iter()
            .map(|g| {
                let t: HashSet<char> = g.chars().filter(|c| *c != '\n').collect();
                return t.len();
            })
            .sum::<usize>()
    );

    println!(
        "Part2: {}",
        groups
            .iter()
            .map(|g| {
                g.lines()
                    .fold(('a'..='z').collect::<HashSet<char>>(), |s, l| {
                        s.intersection(&l.chars().collect()).copied().collect()
                    })
                    .len()
            })
            .sum::<usize>()
    );
}
