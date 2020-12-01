use itertools::Itertools;
fn main() {
    let input: String =
        std::fs::read_to_string("./src/input.txt").expect("Something went wrong reading the INPUT file");
    let report: Vec<i32>  = input.split_ascii_whitespace().map(|x| x.parse().unwrap()).sorted().collect();
    
    let found = report.iter().combinations(2).find(|x| x[0] + x[1] == 2020).unwrap();
    println!("Part1: {}", found[0]*found[1]);
    let found = report.iter().combinations(3).find(|x| x[0] + x[1] + x[2] == 2020).unwrap();
    println!("Part2: {}", found[0]*found[1]*found[2]);
}
