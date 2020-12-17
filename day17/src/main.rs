use itertools::{iproduct, Itertools};
use std::collections::HashSet;
use std::hash::Hash;


type Coord3d = (i32, i32, i32);
type Coord4d = (i32, i32, i32, i32);

trait Cube: Eq + Hash + Clone {
    fn new(x: i32, y: i32) -> Self;

    fn get_neighbours(&self) -> Vec<Self>;

    fn print(active_cubes: &HashSet<Self>);

    fn get_active(self: &Self, active_cubes: &HashSet<Self>) -> bool {
        let c = self
            .get_neighbours()
            .iter()
            .fold(0, |acc, c| match active_cubes.contains(c) {
                true => acc + 1,
                false => acc,
            });
        match active_cubes.contains(self) {
            true => (2..=3).contains(&c),
            false => c == 3,
        }
    }

    fn parse_input(input: &str) -> HashSet<Self> {
        let mut start = HashSet::new();
        for (x, line) in input.lines().enumerate() {
            for (y, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        start.insert(Self::new(x as i32, y as i32));
                    }
                    _ => (),
                }
            }
        }
        return start;
    }

    fn run(start: HashSet<Self>) -> HashSet<Self> {
        let mut next = start.clone();
        for _i in 0..6 {
            next = next
                .iter()
                .cloned()
                .chain(next.iter().flat_map(Self::get_neighbours))
                .unique()
                .filter(|c| c.get_active(&next))
                .collect();
            //println!("After cycle {}", _i + 1);
            //Self::print(&next);
        }
        return next;
    }
}

impl Cube for Coord3d {
    fn new(x: i32, y: i32) -> Self {
        (x, y, 0)
    }

    fn get_neighbours(&self) -> Vec<Self> {
        let me = self.clone();
        return iproduct!(-1..=1, -1..=1, -1..=1)
            .filter_map(|c| match c {
                (0, 0, 0) => None,
                p => Some((me.0 + p.0, me.1 + p.1, me.2 + p.2)),
            })
            .collect();
    }

    fn print(active_cubes: &HashSet<Self>) {
        let xs: Vec<i32> = active_cubes.iter().map(|c| c.0).sorted().collect();
        let ys: Vec<i32> = active_cubes.iter().map(|c| c.1).sorted().collect();
        let zs: Vec<i32> = active_cubes.iter().map(|c| c.2).sorted().collect();

        for z in *zs.iter().next().unwrap()..=*zs.iter().last().unwrap() {
            println!("z = {}", z);
            for y in *ys.iter().next().unwrap()..=*ys.iter().last().unwrap() {
                for x in *xs.iter().next().unwrap()..=*xs.iter().last().unwrap() {
                    print!(
                        "{}",
                        if active_cubes.contains(&(x, y, z)) {
                            '#'
                        } else {
                            '.'
                        }
                    );
                }
                print!("\n");
            }
            print!("\n");
        }
    }
}

impl Cube for Coord4d {
    fn new(x: i32, y: i32) -> Self {
        (x, y, 0, 0)
    }

    fn get_neighbours(&self) -> Vec<Self> {
        let me = self.clone();
        return iproduct!(-1..=1, -1..=1, -1..=1, -1..=1)
            .filter_map(|c| match c {
                (0, 0, 0, 0) => None,
                p => Some((me.0 + p.0, me.1 + p.1, me.2 + p.2, me.3 + p.3)),
            })
            .collect();
    }

    fn print(active_cubes: &HashSet<Self>) {
        let xs: Vec<i32> = active_cubes.iter().map(|c| c.0).sorted().collect();
        let ys: Vec<i32> = active_cubes.iter().map(|c| c.1).sorted().collect();
        let zs: Vec<i32> = active_cubes.iter().map(|c| c.2).sorted().collect();
        let ws: Vec<i32> = active_cubes.iter().map(|c| c.3).sorted().collect();

        for w in *ws.iter().next().unwrap()..=*ws.iter().last().unwrap() {
            for z in *zs.iter().next().unwrap()..=*zs.iter().last().unwrap() {
                println!("z = {}, w = {}", z, w);
                for y in *ys.iter().next().unwrap()..=*ys.iter().last().unwrap() {
                    for x in *xs.iter().next().unwrap()..=*xs.iter().last().unwrap() {
                        print!(
                            "{}",
                            if active_cubes.contains(&(x, y, z, w)) {
                                '#'
                            } else {
                                '.'
                            }
                        );
                    }
                    print!("\n");
                }
                print!("\n");
            }
        }
    }
}

fn main() {
    let raw_input = std::fs::read_to_string("src/input.txt").expect("Error reading the file!");
    println!("{:?}", Cube::run(Coord3d::parse_input(&raw_input)).len());
    println!("{:?}", Cube::run(Coord4d::parse_input(&raw_input)).len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = ".#.
..#
###";
        assert_eq!(Cube::run(Coord3d::parse_input(&input)).len(), 112);
    }

    #[test]
    fn test_part2() {
        let input = ".#.
..#
###";
        assert_eq!(Cube::run(Coord4d::parse_input(&input)).len(), 848);
    }
}
