use itertools::Itertools;
extern crate grabbag;
use grabbag::iter::AccumulateIterator;

fn main() {
    let raw_input = std::fs::read_to_string("src/input.txt").expect("Error reading the file!");
    let numbers = raw_input
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u64>>();

    let target = numbers
        .iter()
        .enumerate()
        .skip(25)
        .find(|(i, x)| {
            return !numbers[i - 25..*i].iter().combinations(2).any(|c| {
                return c.iter().copied().sum::<u64>() == **x;
            });
        })
        .unwrap()
        .1;

    println!("Part1: {}", target);

    let partial_sums: Vec<u64> = numbers
        .clone()
        .into_iter()
        .accumulate(|a, b| a + b)
        .collect();

    let mut l = 0;
    let mut r = 1;
    loop {
        let d = partial_sums[r] - partial_sums[l];
        if d == *target {
            break;
        }
        if d < *target {
            r += 1
        } else {
            l += 1;
        }
    }
    let min = numbers[l..r].iter().min().unwrap();
    let max = numbers[l..r].iter().max().unwrap();
    println!("Part2: {}", *min + *max);
}
