type LinkedList = Vec<Node>;
#[derive(Debug, Clone, Copy)]
struct Node {
    prev: usize,
    next: usize,
}

impl Node {
    fn new(prev: usize, next: usize) -> Self {
        Self { prev, next }
    }
}

fn get_linked_list(array: &Vec<usize>) -> LinkedList {
    let mut list = vec![Node::new(0,0); array.len() + 1];
    array
        .windows(3)
        .for_each(|w| list[w[1]] = Node::new(w[0], w[2]));
    list[array[0]] = Node::new(array[array.len() - 1], array[1]);
    list[array[array.len() - 1]] = Node::new(array[array.len() - 2], array[0]);
    list
}

pub fn part1(input: &str) -> usize {
    let cups: Vec<usize> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let mut crt = cups[0];
    let mut cups = get_linked_list(&cups);
    for _ in 0..100 {
        next(&mut cups, &mut crt);
    }

    let mut res = String::new();
    crt = 1;
    loop {
        crt = cups[crt].next;
        if crt == 1 {
            break;
        }
        res.push_str(&crt.to_string());
    }
    res.parse().unwrap()
}

fn part2(input: &str) -> usize {
    let mut cups: Vec<usize> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let max = cups.iter().copied().max().unwrap();
    cups.extend(max + 1..=1_000_000);

    let mut crt = cups[0];
    let mut cups = get_linked_list(&cups);
    for _ in 0..10_000_000 {
        next(&mut cups, &mut crt);
    }
    cups[1].next * cups[cups[1].next].next
}

fn next(cups: &mut LinkedList, crt: &mut usize) {
    //Pick next 3 cups
    let c1 = cups[*crt].next;
    let c2 = cups[c1].next;
    let c3 = cups[c2].next;

    cups[*crt].next = cups[c3].next;

    //Compute destination cup
    let mut dest = if *crt == 1 { cups.len() - 1 } else { *crt - 1 };
    let picked = vec![c1, c2, c3];
    loop {
        if !picked.contains(&dest) {
            break;
        }
        dest = if dest == 1 { cups.len() - 1 } else { dest - 1 };
    }

    //Insert cups
    let dest_next = cups[dest].next;
    cups[dest].next = c1;
    cups[c1] = Node::new(dest, c2);
    cups[c3] = Node::new(c2, dest_next);
    cups[dest_next].prev = c3;

    //Update crt
    *crt = cups[*crt].next;
}

fn main() {
    const INPUT: &str = "315679824";
    println!("Part1: {}", part1(INPUT));
    println!("Part2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "389125467";

        assert_eq!(part1(&input), 67384529);
    }
    #[test]
    fn test_part2() {
        let input = "389125467";

        assert_eq!(part2(&input), 149245887792);
    }
}
