use defaultmap::DefaultHashMap;

fn is_tree(
    pos: (usize, usize),
    grid: &DefaultHashMap<(usize, usize), char>,
    original_grid_size_x: usize,
) -> bool {
    let x = pos.0 % original_grid_size_x;
    return grid[(x, pos.1)] == '#';
}

fn main() {
    let raw_input = std::fs::read_to_string("src/input.txt").expect("Error reading the file!");
    let original_grid_size_x = raw_input.lines().next().unwrap().len();
    let original_grid_size_y = raw_input.lines().count();

    let mut grid: defaultmap::DefaultHashMap<(usize, usize), char> =
        defaultmap::DefaultHashMap::new('.');
    for (i, line) in raw_input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                grid.insert((j, i), c);
            }
        }
    }

    let mut slopes: Vec<(usize, usize, usize, usize, usize)> = vec![
        (0, 0, 1, 1, 0),
        (0, 0, 3, 1, 0),
        (0, 0, 5, 1, 0),
        (0, 0, 7, 1, 0),
        (0, 0, 1, 2, 0),
    ];
    loop {

        //TEST FOR STOP
        if slopes.iter().all(|x| x.1 > original_grid_size_y) {
            break;
        }

        // ADVANCE EACH SLOPE AND TEST FOR TREES
        //------------------------------------
        slopes.iter_mut().for_each(|x| {
            x.0 += x.2;
            x.1 += x.3;
            if is_tree((x.0, x.1), &grid, original_grid_size_x) {
                x.4 += 1;
            }
        });
    }

    println!("Part1: {}", slopes[1].4);
    println!("Part2: {}", slopes.iter().fold(1, |acc, x| acc * x.4));
}
