use counter::Counter;
use defaultmap::DefaultHashMap;
use itertools::{zip, Itertools};

fn parse_input(input: &str) -> Vec<i32> {
    return input
        .lines()
        .map(|l| l.parse().unwrap())
        .sorted()
        .collect::<Vec<i32>>();
}

fn count_permutations(voltages: Vec<i32>) -> u64 {
    let mut voltages = voltages.clone();
    voltages.push(*voltages.iter().last().unwrap());
    let mut c: DefaultHashMap<i32, u64> = DefaultHashMap::new(0);
    c[0] = 1;
    voltages.iter().for_each(|v| c[*v] = c[*v - 1] + c[*v - 2] + c[*v - 3]);
    return c[voltages.iter().last().unwrap()];
}

fn main() {
    let raw_input = std::fs::read_to_string("src/input.txt").expect("Error reading the file!");
    let voltages = parse_input(raw_input.as_str());
    println!("{:?}", voltages);
    let counts = zip(&voltages[1..], &voltages[0..voltages.len() - 1])
        .map(|(a, b)| a - b)
        .collect::<Counter<_>>();
    println!("Part1: {:?}", (counts[&3] + 1) * (counts[&1] + 1));

    println!("Part2: {}", count_permutations(voltages));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_permutations1() {
        let input = "16
10
15
5
1
11
7
19
6
12
4";
        let vs = parse_input(input);
        assert_eq!(count_permutations(vs), 8);
    }

    #[test]
    fn test_count_permutations2() {
        let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        let vs = parse_input(input);
        assert_eq!(
            count_permutations(vs),
            19208
        );
    }
}
