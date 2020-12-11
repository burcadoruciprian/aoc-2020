use std::collections::HashMap;

type SeatsGrid = HashMap<(i32, i32), char>;
type Pos = (i32, i32);
type FlipSeatFn = fn(grid: &SeatsGrid, seat: Pos) -> bool;

#[cfg_attr(rustfmt, rustfmt_skip)]
static DIRECTIONS: [Pos; 8] = [(-1,-1),(0,-1),(1,-1),
                                     (-1, 0),(1,0),
                                     (-1,1),(0,1),(1,1)];

fn parse_input(input: &str) -> SeatsGrid {
    let mut grid: SeatsGrid = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.insert((x as i32, y as i32), c);
        }
    }
    return grid;
}

fn print_grid(grid: &SeatsGrid) {
    let x_max = grid.keys().map(|(x, _)| x).max().unwrap();
    let y_max = grid.keys().map(|(_, y)| y).max().unwrap();

    for y in 0..=*y_max {
        for x in 0..=*x_max {
            print!("{}", grid[&(x, y)])
        }
        print!("\n");
    }

    println!("\n----------------------------------\n");
}

fn should_flip_seat1(grid: &SeatsGrid, seat: Pos) -> bool {
    if grid[&seat] == '.' {
        return false;
    }

    let mut adiacent = DIRECTIONS
        .iter()
        .map(|&(dx, dy)| (seat.0 + dx, seat.1 + dy))
        .filter_map(|(x, y)| grid.get(&(x, y)));

    match grid[&seat] {
        'L' => adiacent.all(|&c| c != '#'),
        '#' => adiacent.filter(|&&c| c == '#').count() >= 4,
        _ => panic!(),
    }
}

fn should_flip_seat2(grid: &SeatsGrid, seat: Pos) -> bool {
    if grid[&seat] == '.' {
        return false;
    }

    let seek_seat = |crt_seat: Pos, direction: Pos| -> Option<char> {
        let (mut x, mut y) = crt_seat;
        let (dx, dy) = direction;
        loop {
          x += dx;
          y += dy;
          match grid.get(&(x, y)){
            Some('.') => (),
            Some(&c) => return Some(c),
            None => break,
          }
        }
        None
    };

    let mut neighbours = DIRECTIONS
        .iter()
        .filter_map(|d| seek_seat(seat, *d));

    match grid[&seat] {
        'L' => neighbours.all(|c| c != '#'),
        '#' => neighbours.filter(|&c| c == '#').count() >= 5,
        _ => panic!(),
    }
}

fn next_state(grid: &mut SeatsGrid, f: FlipSeatFn) -> (i32, i32) {
    let mut e = 0;
    let mut o = 0;

    let mut next_grid = grid.clone();
    next_grid
        .iter_mut()
        .for_each(|(k, v)| match (&v, f(&grid, *k)) {
            ('#', true) => {
                *v = 'L';
                e += 1
            }
            ('#', false) => o += 1,
            ('L', true) => {
                *v = '#';
                o += 1
            }
            ('L', false) => e += 1,
            ('.', _) => (),
            _ => panic!(),
        });
    *grid = next_grid;
    return (o, e);
}

fn run(grid: SeatsGrid, f: FlipSeatFn) -> i32 {
    let mut seats_state = (0, 0);
    let mut tmp = grid;
    loop {
        let crt = next_state(&mut tmp, f);
        //print_grid(&tmp);
        if seats_state == crt {
            break;
        }
        seats_state = crt;
    }
    return seats_state.0;
}

fn main() {
    let raw_input = std::fs::read_to_string("src/input.txt").expect("Error reading the file!");
    let grid = parse_input(raw_input.as_str());
    println!("Part1: {}", run(grid.clone(), should_flip_seat1));
    println!("Part2: {}", run(grid, should_flip_seat2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        assert_eq!(run(parse_input(input), should_flip_seat1), 37);
    }

        #[test]
    fn test_part2() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        assert_eq!(run(parse_input(input), should_flip_seat2), 26);
    }
}
