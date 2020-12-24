use counter::Counter;

type TileContent = [[char; 10]; 10];
#[derive(Debug)]
struct Tile {
    id: u32,
    tile: TileContent,
    edges: (u32, u32, u32, u32),
}

impl Tile {
    fn print(&self) {
        self.tile
            .iter()
            .for_each(|l| println!("{}", l.iter().collect::<String>()))
    }

    fn rotate_clockwise(&mut self) {
        self.tile.reverse();
        for i in 1..self.tile.len() {
            let (left, right) = self.tile.split_at_mut(i);
            for (j, left_item) in left.iter_mut().enumerate().take(i) {
                std::mem::swap(&mut left_item[i], &mut right[0][j]);
            }
        }

        std::mem::swap(&mut self.edges.0, &mut self.edges.3);
        std::mem::swap(&mut self.edges.0, &mut self.edges.1);
        std::mem::swap(&mut self.edges.1, &mut self.edges.2);
    }

    fn flip_vertical(&mut self) {
        std::mem::swap(&mut self.edges.1, &mut self.edges.3);
        self.tile.iter_mut().for_each(|t| t.reverse());
    }

    fn flip_horizontal(&mut self) {
        std::mem::swap(&mut self.edges.0, &mut self.edges.2);
        self.tile.reverse();
    }

    fn get_encoded_edges(tile: &TileContent) -> (u32, u32, u32, u32) {
        let t = Tile::encode(*tile.iter().next().unwrap());
        let b = Tile::encode(*tile.iter().last().unwrap());

        let mut col = ['.'; 10];
        tile.iter().enumerate().for_each(|(i, l)| col[i] = l[0]);
        let l = Tile::encode(col);

        tile.iter().enumerate().for_each(|(i, l)| col[i] = l[9]);
        let r = Tile::encode(col);

        (t, r, b, l)
    }

    fn encode(line: [char; 10]) -> u32 {
        let bin = line
            .iter()
            .map(|c| match c {
                '#' => '1',
                _ => '0',
            })
            .collect::<String>();
        let n = u32::from_str_radix(bin.as_str(), 2).unwrap();
        let n_r = u32::from_str_radix(bin.chars().rev().collect::<String>().as_str(), 2).unwrap();
        return std::cmp::min(n, n_r);
    }
}

type Tiles = Vec<Tile>;

fn parse_input(input: &str) -> Tiles {
    return input
        .split("\n\n")
        .map(|t| parse_tile(t))
        .collect::<Tiles>();
}

fn parse_tile(input: &str) -> Tile {
    let mut id = 0;
    let mut tile: TileContent = [['.'; 10]; 10];
    input.lines().enumerate().for_each(|(i, l)| {
        if i == 0 {
            id = parse_id_line(l);
        } else {
            l.chars().enumerate().for_each(|(j, c)| tile[i - 1][j] = c);
        }
    });
    Tile {
        id: id,
        tile: tile,
        edges: Tile::get_encoded_edges(&tile),
    }
}

fn parse_id_line(line: &str) -> u32 {
    let re = regex::Regex::new(r"Tile (\d+):").unwrap();
    return re
        .captures(line)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<u32>()
        .unwrap();
}

fn part1(input: &str) -> u64 {
    let tiles = parse_input(input);

    let counter = tiles
        .iter()
        .flat_map(|t| {
            vec![
                t.edges.0,
                t.edges.1,
                t.edges.2,
                t.edges.3,
            ]
        })
        .collect::<Counter<_>>();

    return tiles.into_iter().fold(1, |acc, tl| {
        let (t, b, l, r) = tl.edges;
        if counter[&t] + counter[&b] + counter[&l] + counter[&r] == 6 {
            return acc * tl.id as u64;
        } else {
            return acc;
        }
    });
}

fn part2(input: &str) -> u64 {
    let tiles = parse_input(input);

    let counter = tiles
        .iter()
        .flat_map(|t| {
            vec![
                t.edges.0,
                t.edges.1,
                t.edges.2,
                t.edges.3,
            ]
        })
        .collect::<Counter<_>>();

    let corner = tiles
        .iter()
        .find(|tl| {
            let (t, r, b, l) = tl.edges;
            return counter[&t] + counter[&b] + counter[&l] + counter[&r] == 6;
        })
        .unwrap();

    let edge_size = (tiles.len() as f64).sqrt() as u32;
    dbg!(edge_size);

    0
}

fn main() {
    let raw_input = std::fs::read_to_string("src/input.txt").expect("Error reading the file!");
    //println!("Part1 {}", part1(raw_input.as_str()));
    //println!("Part2 {}", part2(raw_input.as_str()));



}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test() {
        let input = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";

        assert_eq!(part1(input), 20899048083289);
        assert_eq!(part2(input), 0);
    }
}
