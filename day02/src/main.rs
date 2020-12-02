fn main() {
    //Read an parse the input
    let mut passwords: Vec<(usize, usize, char, String)> = Vec::new();
    let raw_input = std::fs::read_to_string("src/input.txt").expect("Error reading the file!");
    for line in raw_input.lines() {
        let items: Vec<&str> = line
            .split(&['-', ':', ' '][..])
            .filter(|c| !c.is_empty())
            .collect();

        passwords.push((
            items[0].parse::<usize>().unwrap() as usize,
            items[1].parse::<usize>().unwrap() as usize,
            items[2].chars().next().unwrap(),
            String::from(items[3]),
        ));
    }

    let part1 = passwords.iter().fold(0, |acc, x| {
        let c = x.3.matches(x.2).count();
        if x.0 <= c && c <= x.1 {
            return acc + 1;
        } else {
            return acc;
        };
    });
    println!("Part1: {}", part1);

    let part2 = passwords.iter().fold(0, |acc, x| {
        let c =
            x.3.match_indices(x.2)
                .filter(|v| v.0 + 1 == x.0 || v.0 + 1 == x.1)
                .count();
        if c == 1 {
            return acc + 1;
        } else {
            return acc;
        }
    });
    println!("Part1: {}", part2)
}
