use itertools::Itertools;

fn decode_id(borading_pass: &str) -> i32 {
    let (row, col) = borading_pass.split_at(7);
    decode_row(row) * 8 + decode_col(col)
}

fn decode_row(s: &str) -> i32 {
    s.chars()
        .fold(0..=127, |range, c| {
            let mid = (range.start() + range.end()) / 2;
            match c {
                'F' => *range.start()..=mid,
                'B' => mid..=*range.end(),
                _ => panic!(),
            }
        })
        .last()
        .unwrap()
}

fn decode_col(s: &str) -> i32 {
    s.chars()
        .fold(0..=7, |range, c| {
            let mid = (range.start() + range.end()) / 2;
            match c {
                'L' => *range.start()..=mid,
                'R' => mid..=*range.end(),
                _ => panic!(),
            }
        })
        .last()
        .unwrap()
}

fn main() {
    let raw_input = std::fs::read_to_string("src/input.txt").expect("Error reading the file!");

    let seats: Vec<i32> = raw_input.lines().map(decode_id).sorted().collect();

    println!("Part1: {}", seats.iter().last().unwrap());

    let first_seat_id = seats.iter().next().unwrap();
    let missing_seat = seats
        .iter()
        .enumerate()
        .find(|(i, v)| *i as i32 + first_seat_id != **v)
        .unwrap();
    println!("Part2: {}", missing_seat.1 - 1);
}
