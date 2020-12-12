type Pos = (i32, i32);
fn main() {
    let raw_input = std::fs::read_to_string("src/input.txt").expect("Error reading the file!");
    let instructions = raw_input
        .lines()
        .map(|line| (line.chars().next().unwrap(), line[1..].parse().unwrap()))
        .collect::<Vec<(char, i32)>>();

    let end1 = run1(instructions.clone());
    println!("Part1 {}", end1.0.abs() + end1.1.abs() );
    let end2 = run2(instructions);
    println!("Part1 {}", end2.0.abs() + end2.1.abs() );
}

fn mv(crt_pos: Pos, direction: char, ammount: i32) -> Pos {
    let (mut x, mut y) = crt_pos;
    match direction {
        'N' => y += ammount,
        'S' => y -= ammount,
        'E' => x += ammount,
        'W' => x -= ammount,
        _ => panic!(),
    }
    return (x, y);
}

fn rt(crt_orientation: Pos, direction: char, degrees: i32) -> Pos {
    let (mut x, mut y) = crt_orientation;
    for _ in 1..=degrees / 90 {
        match direction {
            'L' => {
                let tx = x;
                let ty = y;
                x = -ty;
                y = tx
            }
            'R' => {
                let tx = x;
                let ty = y;
                x = ty;
                y = -tx
            }
            _ => panic!(),
        }
    }
    return (x, y);
}

fn fwd(crt_pos: Pos, orientation: Pos, ammount: i32) -> Pos {
    return (
        crt_pos.0 + orientation.0 * ammount,
        crt_pos.1 + orientation.1 * ammount,
    );
}

fn run1(instructions: Vec<(char, i32)>) -> Pos {
    let mut crt_pos = (0, 0);
    let mut crt_orientation = (1, 0);
    for (c, amount) in instructions {
        match c {
            'N' | 'S' | 'E' | 'W' => crt_pos = mv(crt_pos, c, amount),
            'L' | 'R' => crt_orientation = rt(crt_orientation, c, amount),
            'F' => crt_pos = fwd(crt_pos, crt_orientation, amount),
            _ => panic!(),
        }
    }

    return crt_pos;
}

fn run2(instructions: Vec<(char, i32)>) -> Pos {
    let mut ship_crt_pos = (0, 0);
    let mut way_crt_pos = (10, 1);
    for (c, amount) in instructions {
        match c {
            'N' | 'S' | 'E' | 'W' => way_crt_pos = mv(way_crt_pos, c, amount),
            'L' | 'R' => way_crt_pos = rt(way_crt_pos, c, amount),
            'F' => ship_crt_pos = fwd(ship_crt_pos, way_crt_pos, amount),
            _ => panic!(),
        }
    }

    return ship_crt_pos;
}
