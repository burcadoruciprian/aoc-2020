use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<(&str, i32)> {
    return input
        .lines()
        .map(|l| (&l[..3], l[4..].parse::<i32>().unwrap()))
        .collect::<Vec<(&str, i32)>>();
}

fn run(program: &Vec<(&str, i32)>) -> Result<i32, i32> {
    let mut acc = 0;
    let mut ip = 0;

    let mut history: HashSet<usize> = HashSet::new();
    loop {
        if ip >= program.len() {
            break;
        }
        //Test for loop
        if history.contains(&ip) {
            return Err(acc);
        }
        history.insert(ip);

        match program[ip] {
            ("acc", arg) => {
                acc += arg;
                ip += 1
            }
            ("nop", _) => ip += 1,
            ("jmp", arg) => ip = (ip as i32 + arg) as usize,
            _ => panic!("Unknown command"),
        }
    }

    Ok(acc)
}

fn fix_loop(program: Vec<(&str, i32)>) -> Result<i32, ()> {
    let mut program = program;
    for i in 0..program.len() {
        swap(&mut program[i]);

        match run(&program) {
            Ok(acc) => return Ok(acc),
            Err(_) => (),
        }

        swap(&mut program[i]);
    }
    Err(())
}

fn swap(op: &mut (&str, i32)) {
    match op {
        ("nop", _) => {
            op.0 = "jmp";
        }
        ("jmp", _) => {
            op.0 = "nop";
        }
        _ => (),
    }
}

fn main() {
    let raw_input = std::fs::read_to_string("src/input.txt").expect("Error reading the file!");
    let program = parse_input(raw_input.as_str());

    println!(
        "Part1: {}",
        match run(&program) {
            Ok(acc) => acc,
            Err(acc) => acc,
        }
    );
    println!(
        "Part2: {}",
        match fix_loop(program) {
            Ok(acc) => acc,
            Err(_) => panic!("ERROR: Could not fix loop"),
        }
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_loop() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        assert_eq!(run(&parse_input(input)), Err(5));
    }
    #[test]
    fn test_fix_loop() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        assert_eq!(fix_loop(parse_input(input)), Ok(8));
    }
}
