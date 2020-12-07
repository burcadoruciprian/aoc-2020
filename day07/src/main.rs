use regex::Regex;
use std::collections::HashMap;

fn parse_input(input: String) -> HashMap<String, Vec<(String, usize)>> {
    let bag_re = Regex::new(r"^(.+?) bags").unwrap();
    let inner_bag_re = Regex::new(r"(\d+?) (.+?) bag").unwrap();
    return input
        .lines()
        .map(|line| {
            let bag = bag_re.captures(line).unwrap()[1].to_string();
            let inner_bags = inner_bag_re
                .captures_iter(line)
                .map(|capture| (capture[2].to_string(), capture[1].parse().unwrap()))
                .collect();
            (bag, inner_bags)
        })
        .collect();
}

fn contains_gold_bag(bags: &HashMap<String, Vec<(String, usize)>>, bag: &str) -> bool {
    bag == "shiny gold" || bags[bag].iter().any(|(b, _)| contains_gold_bag(bags, b))
}

fn gold_bag_total_bags(bags: &HashMap<String, Vec<(String, usize)>>, bag: &str) -> usize {
    1 + bags[bag]
        .iter()
        .map(|(b, c)| c * gold_bag_total_bags(bags, b))
        .sum::<usize>()
}

fn main() {
    let raw_input = std::fs::read_to_string("src/input.txt").expect("Error reading the file!");
    let bags = parse_input(raw_input);

    println!(
        "Part1: {}",
        bags.keys()
            .filter(|&bag| contains_gold_bag(&bags, bag))
            .count()
            - 1
    );

    println!("Part2: {}", gold_bag_total_bags(&bags, "shiny gold") - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_gold_bag() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        let bags = parse_input(input.to_string());
        assert_eq!(
            bags.keys()
                .filter(|&bag| contains_gold_bag(&bags, bag))
                .count()
                - 1,
            4
        );
    }
    #[test]
    fn test_gold_bag_total_bags() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        let bags = parse_input(input.to_string());
        assert_eq!(gold_bag_total_bags(&bags, "shiny gold") - 1, 32);

       let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        let bags = parse_input(input.to_string());
        assert_eq!(gold_bag_total_bags(&bags, "shiny gold") - 1, 126);
    }
}
