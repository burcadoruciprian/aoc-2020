use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

type Rules = HashMap<u32, Vec<Vec<String>>>;

fn parse_input(input: &str) -> (Rules, Vec<String>) {
    let (raw_rules, messages) = input
        .split("\n\n")
        .map(|s| s.lines())
        .collect_tuple()
        .unwrap();

    let mut rules: HashMap<u32, Vec<Vec<String>>> = Rules::new();
    raw_rules.for_each(|r| {
        let (id, content) = r.split(": ").collect_tuple().unwrap();
        if content.chars().nth(0) == Some('"') {
            rules.insert(
                id.parse().unwrap(),
                vec![vec![content.chars().nth(1).unwrap().to_string()]],
            );
        } else {
            rules.insert(
                id.parse().unwrap(),
                content
                    .split(" | ")
                    .map(|t| t.split(" ").map(|c| c.to_string()).collect::<Vec<String>>())
                    .collect(),
            );
        }
    });

    (rules, messages.map(|l| l.to_string()).collect())
}

fn main() {
    let input = std::fs::read_to_string("src/input.txt").expect("Error reading the file!");
    let (mut rules, messages) = parse_input(input.as_str());

    let rule_0: VecDeque<String> =
        VecDeque::from(rules.get(&0).unwrap().clone().get(0).unwrap().clone());
    println!(
        "Part1: {}",
        messages.iter().fold(0, |acc, m| {
            match is_match(m.clone(), rule_0.clone(), &rules) {
                false => acc,
                true => acc + 1,
            }
        })
    );

    *rules.get_mut(&8).unwrap() = vec![
        vec![42.to_string()],
        vec![42.to_string(), 8.to_string()],
    ];
    *rules.get_mut(&11).unwrap() = vec![
        vec![42.to_string(), 31.to_string()],
        vec![42.to_string(), 11.to_string(), 31.to_string()],
    ];

    println!(
        "Part2: {}",
        messages.iter().fold(0, |acc, m| {
            match is_match(m.clone(), rule_0.clone(), &rules) {
                false => acc,
                true => acc + 1,
            }
        })
    );
}

fn is_match(message: String, rule: VecDeque<String>, rules: &Rules) -> bool {
    if rule.len() > message.len() {
        return false;
    } else if rule.len() == 0 || message.len() == 0 {
        return rule.len() == 0 && message.len() == 0;
    }

    let mut rule = rule.clone();
    let mut message = message.clone();
    let s = rule.pop_front().unwrap();
    match s.parse::<u32>() {
        Err(_) => {
            if message.starts_with(&s) {
                message.remove(0);
                return is_match(message, rule.clone(), &rules);
            }
        }
        Ok(n) => match rules.get(&n) {
            Some(next_rules) => {
                for mut n in next_rules.clone() {
                    n.extend(rule.clone());
                    if is_match(message.clone(), VecDeque::from(n), &rules) {
                        return true;
                    }
                }
            }
            _ => panic!(),
        },
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_match() {
        let input = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";

        let (rules, messages) = parse_input(input);

        let rule_0: VecDeque<String> =
            VecDeque::from(rules.get(&0).unwrap().clone().get(0).unwrap().clone());
        assert_eq!(
            messages.iter().fold(0, |acc, m| {
                match is_match(m.clone(), rule_0.clone(), &rules) {
                    false => acc,
                    true => acc + 1,
                }
            }),
            2
        );
    }
}
