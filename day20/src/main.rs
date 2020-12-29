use counter::Counter;
use std::collections::HashSet;
use tuple::*;

type TileContent = Vec<Vec<char>>;
#[derive(Debug, Clone)]
struct Tile {
    id: u32,
    tile: TileContent,
    edges: (u32, u32, u32, u32),
}

impl Tile {
    fn new() -> Self {
        Self {
            id: 0,
            tile: TileContent::new(),
            edges: (0, 0, 0, 0),
        }
    }
    fn print(&self) {
        self.tile
            .iter()
            .for_each(|l| println!("{}", l.iter().collect::<String>()))
    }

    fn rotate(&mut self) {
        self.tile.reverse();
        for i in 1..self.tile.len() {
            let (left, right) = self.tile.split_at_mut(i);
            for (j, left_item) in left.iter_mut().enumerate().take(i) {
                std::mem::swap(&mut left_item[i], &mut right[0][j]);
            }
        }

        let tmp = self.edges.3;
        self.edges.3 = self.edges.2;
        self.edges.2 = self.edges.1;
        self.edges.1 = self.edges.0;
        self.edges.0 = tmp;
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
        let t = Tile::encode(tile.first().unwrap());
        let b = Tile::encode(tile.last().unwrap());
        let l = Tile::encode(&tile.iter().map(|v| *v.first().unwrap()).collect());
        let r = Tile::encode(&tile.iter().map(|v| *v.last().unwrap()).collect());

        (t, r, b, l)
    }

    fn encode(line: &Vec<char>) -> u32 {
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

    fn get_top(&self) -> (u32, String) {
        return (
            self.edges.0,
            self.tile.first().unwrap().iter().collect::<String>(),
        );
    }

    fn get_bottom(&self) -> (u32, String) {
        return (
            self.edges.2,
            self.tile.last().unwrap().iter().collect::<String>(),
        );
    }

    fn get_left(&self) -> (u32, String) {
        return (
            self.edges.3,
            self.tile
                .iter()
                .map(|v| *v.first().unwrap())
                .collect::<String>(),
        );
    }

    fn get_right(&self) -> (u32, String) {
        return (
            self.edges.1,
            self.tile
                .iter()
                .map(|v| *v.last().unwrap())
                .collect::<String>(),
        );
    }

    fn make_it_match_right(&self, other: &mut Tile) {
        let (s_r, s_rv) = self.get_right();

        loop {
            let (o_l, o_lv) = other.get_left();
            if s_r == o_l && s_rv == o_lv {
                return;
            }

            if s_r == o_l && s_rv != o_lv {
                other.flip_horizontal();
                continue;
            }

            other.rotate();
        }
    }

    fn make_it_match_bottom(&self, other: &mut Tile) {
        let (s_b, s_bv) = self.get_bottom();

        loop {
            let (o_t, o_tv) = other.get_top();
            if s_b == o_t && s_bv == o_tv {
                return;
            }

            if s_b == o_t && s_bv != o_tv {
                other.flip_vertical();
                continue;
            }

            other.rotate();
        }
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
    let mut tile = TileContent::new();
    input.lines().enumerate().for_each(|(i, l)| {
        if i == 0 {
            id = parse_id_line(l);
        } else {
            tile.push(l.chars().collect::<Vec<char>>());
        }
    });
    Tile {
        id: id,
        tile: tile.clone(),
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
        .flat_map(|t| vec![t.edges.0, t.edges.1, t.edges.2, t.edges.3])
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

fn part2(input: &str) -> u32 {
    let mut tiles = parse_input(input);

    let counter = tiles
        .iter()
        .flat_map(|t| vec![t.edges.0, t.edges.1, t.edges.2, t.edges.3])
        .collect::<Counter<_>>();

    let mut top_left_corner = tiles
        .iter()
        .find(|tl| {
            let (t, r, b, l) = tl.edges;
            return counter[&t] + counter[&b] + counter[&l] + counter[&r] == 6;
        })
        .unwrap()
        .clone();

    //dbg!(top_left_corner.clone());

    while counter[&top_left_corner.edges.0] != 1 || counter[&top_left_corner.edges.3] != 1 {
        top_left_corner.rotate();
    }

    let mut puzzle: Vec<Vec<Tile>> = Vec::new();
    puzzle.push(vec![top_left_corner.clone()]);
    tiles.remove(
        tiles
            .iter()
            .position(|x| x.id == top_left_corner.id)
            .unwrap(),
    );

    loop {
        if tiles.is_empty() {
            break;
        }

        let crt = puzzle.last().unwrap().last().unwrap();
        let (r, _) = crt.get_right();
        let next_right = tiles
            .iter()
            .find(|x| x.edges.0 == r || x.edges.1 == r || x.edges.2 == r || x.edges.3 == r);

        if next_right.is_none() {
            //Finished a row , start another
            let crt = puzzle.last().unwrap().first().unwrap();
            let (b, _) = crt.get_bottom();
            let next_bottom = tiles
                .iter()
                .find(|x| x.edges.0 == b || x.edges.1 == b || x.edges.2 == b || x.edges.3 == b)
                .expect("Cannot find any tiles that match");

            let mut tmp = next_bottom.clone();
            crt.make_it_match_bottom(&mut tmp);
            puzzle.push(vec![tmp]);
            tiles.remove(tiles.iter().position(|x| x.id == next_bottom.id).unwrap());
            continue;
        }

        let mut tmp = next_right.unwrap().clone();
        crt.make_it_match_right(&mut tmp);
        puzzle.last_mut().unwrap().push(tmp);
        tiles.remove(
            tiles
                .iter()
                .position(|x| x.id == next_right.unwrap().id)
                .unwrap(),
        );
    }

    puzzle.iter().for_each(|x| {
        println!("{:?}", x.iter().map(|z| z.id).collect::<Vec<u32>>());
    });

    //Remove borders
    puzzle.iter_mut().flatten().for_each(|t| {
        t.tile.remove(0);
        t.tile.remove(t.tile.len() - 1);
        t.tile.iter_mut().for_each(|l| {
            l.remove(0);
            l.remove(l.len() - 1);
        });
    });

    //Merge tiles into picture
    let mut picture: Tile = Tile::new();
    puzzle.iter().for_each(|l| {
        let size = l[0].tile.len();
        let mut acc: TileContent = vec![vec![]; size];
        l.iter().for_each(|t| {
            t.tile.iter().enumerate().for_each(|(i, v)| {
                acc[i].extend(v);
            });
        });
        picture.tile.extend(acc);
    });

    picture.print();

    const MONSTER: [&str; 3] = [
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",
    ];

    let monster_coords = MONSTER
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .map(move |(j, _)| (i as i32 - 1, j as i32))
        })
        .collect::<HashSet<_>>();

    let total = picture.tile.iter().flatten().filter(|&&c| c == '#').count() as u32;
    loop {
      for i in 0..4 {
        match count_monsters(picture.tile.clone(), &monster_coords) {
            0 => picture.rotate(),
            m => return total - (m * monster_coords.len()) as u32,
        }
      }

      picture.flip_horizontal();
    }
}

fn count_monsters(image: TileContent, monster_coords: &HashSet<(i32, i32)>) -> usize {

  let positions = image
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, &c)| c == '#')
                .map(move |(j, _)| (i as i32, j as i32))
        })
        .collect::<HashSet<_>>();


    positions
        .iter()
        .filter(|(i, j)| {
            monster_coords
                .iter()
                .map(|(di, dj)| (i + di, j + dj))
                .all(|pos| positions.contains(&pos))
        })
        .count()
}

fn main() {
    let raw_input = std::fs::read_to_string("src/input.txt").expect("Error reading the file!");
    //println!("Part1 {}", part1(raw_input.as_str()));
    println!("Part2 {}", part2(raw_input.as_str()));
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

        //assert_eq!(part1(input), 20899048083289);
        assert_eq!(part2(input), 273);
    }
}
