use counter::Counter;
use regex::Regex;

#[derive(Debug, Clone)]
struct TicketField {
    name: String,
    r1: (u64, u64),
    r2: (u64, u64),
}

impl TicketField {
    fn new(name: String, r1: (u64, u64), r2: (u64, u64)) -> TicketField {
        TicketField {
            name: name,
            r1: r1,
            r2: r2,
        }
    }

    fn contains(&self, value: u64) -> bool {
        return (value >= self.r1.0 && value <= self.r1.1)
            || (value >= self.r2.0 && value <= self.r2.1);
    }
}

fn parse_ticket_field_line(input: &str) -> TicketField {
    let re = Regex::new(r"(.+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    let captures = re.captures(input).unwrap();
    return TicketField::new(
        captures[1].to_string(),
        (captures[2].parse().unwrap(), captures[3].parse().unwrap()),
        (captures[4].parse().unwrap(), captures[5].parse().unwrap()),
    );
}

fn parse_input(input: &str) -> (Vec<TicketField>, Vec<u64>, Vec<Vec<u64>>) {
    let mut ticket_fields: Vec<TicketField> = Vec::new();
    let mut your_ticket: Vec<u64> = Vec::new();
    let mut nearby_tickets: Vec<Vec<u64>> = Vec::new();

    let mut nearby_tickets_next: Option<bool> = None;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        if line.starts_with("your ticket:") {
            nearby_tickets_next = Some(false);
            continue;
        }

        if line.starts_with("nearby tickets:") {
            nearby_tickets_next = Some(true);
            continue;
        }

        if nearby_tickets_next.is_none() {
            ticket_fields.push(parse_ticket_field_line(line));
            continue;
        }

        if nearby_tickets_next == Some(false) {
            your_ticket = line
                .split(',')
                .filter_map(|c| c.parse::<u64>().ok())
                .collect();
        }

        if nearby_tickets_next == Some(true) {
            nearby_tickets.push(
                line.split(',')
                    .filter_map(|c| c.parse::<u64>().ok())
                    .collect(),
            );
        }
    }

    (ticket_fields, your_ticket, nearby_tickets)
}

fn part1(ticket_fields: Vec<TicketField>, nearby_tickets: Vec<Vec<u64>>) -> u64 {
    let error_rate = nearby_tickets.into_iter().flatten().fold(0, |acc, v| {
        if !ticket_fields.iter().any(|f| f.contains(v)) {
            return acc + v;
        } else {
            return acc;
        }
    });
    return error_rate;
}

fn part2(
    ticket_fields: Vec<TicketField>,
    your_ticket: Vec<u64>,
    nearby_tickets: Vec<Vec<u64>>,
) -> u64 {
    let nearby_tickets: Vec<Vec<u64>> = nearby_tickets
        .into_iter()
        .filter(|t| {
            t.iter()
                .all(|v| ticket_fields.iter().any(|f| f.contains(*v)))
        })
        .collect();

    let nearby_tickets_count = nearby_tickets.clone().len();
    //dbg!(nearby_tickets_count);

    let mut counters: Vec<Counter<String>> = vec![Counter::new(); your_ticket.len()];
    nearby_tickets.into_iter().for_each(|t| {
        t.iter().enumerate().for_each(|(i, v)| {
            counters[i].update(ticket_fields.iter().filter_map(|f| match f.contains(*v) {
                true => Some(f.name.clone()),
                false => None,
            }))
        });
        //dbg!(t);
        //dbg!(counters.clone());
    });

    let ticket_candidates: Vec<Vec<String>> = counters
        .into_iter()
        .map(|c| {
            c.most_common()
                .into_iter()
                .filter_map(|v| match v.1 == nearby_tickets_count {
                    true => Some(v.0.clone()),
                    false => None,
                })
                .collect::<Vec<String>>()
        })
        .collect();

    let mut ticket_format: Vec<String> = vec!["".to_string(); your_ticket.len()];
    let mut step = 1usize;
    loop {
        if ticket_format.iter().all(|s| !s.is_empty()) {
            break;
        }

        ticket_candidates.iter().enumerate().for_each(|(i, c)| {
            if c.len() != step {
                return;
            }

            let found = c.iter().find(|n| !ticket_format.contains(n)).unwrap();
            ticket_format[i] = found.clone();
        });

        step += 1;
    }
    dbg!(ticket_format.clone());
    return your_ticket.iter().enumerate().fold(1, |p, (i, v)| {
        match ticket_format[i].starts_with("departure") {
            true => p * v,
            false => p,
        }
    });
}

fn main() {
    let raw_input = std::fs::read_to_string("src/input.txt").expect("Error reading the file!");
    let (ticket_fields, your_ticket, nearby_tickets) = parse_input(raw_input.as_str());

    println!(
        "Part1: {}",
        part1(ticket_fields.clone(), nearby_tickets.clone())
    );
    println!(
        "Part2: {}",
        part2(ticket_fields, your_ticket, nearby_tickets)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
        let (ranges, _, nearby_tickets) = parse_input(input);
        assert_eq!(part1(ranges, nearby_tickets), 71);
    }

    #[test]
    fn test_part2() {
        let input = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";
        let (ranges, ticket, nearby_tickets) = parse_input(input);
        assert_ne!(part2(ranges, ticket, nearby_tickets), 0);
    }
}
