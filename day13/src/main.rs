use num::Integer;

const TIMESTAMP: u64 = 1000510;
const SCHEDULE_INPUT: &str = "19,x,x,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,x,523,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,17,13,x,x,x,x,x,x,x,x,x,x,29,x,853,x,x,x,x,x,37,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,23";

fn main() {
    println!("Part1: {}", part1());
    println!("Part2: {}", part2(SCHEDULE_INPUT));
}

fn part1() -> u64 {
    let (id, w) = SCHEDULE_INPUT
        .split(',')
        .filter_map(|x| x.parse::<u64>().ok())
        .map(|x| (x, x - TIMESTAMP % x))
        .min_by_key(|x| x.1)
        .unwrap();
    return id * w;
}

fn part2(input: &str) -> u64 {
    let schedules: Vec<(u64, u64)> = input
        .split(',')
        .enumerate()
        .filter_map(|(i, s)| match s.parse::<u64>() {
            Ok(c) => Some((i as u64, c)),
            Err(_) => None,
        })
        .collect();
    let mut timestamp: u64 = 0;
    let mut inc: u64 = 1;

    for (offset, schedule) in schedules {
        // Find suitable timestamp that fulfills the offset requirement
        while (timestamp + offset) % schedule != 0 {
            timestamp += inc;
        }

        //GOTCHA!!!!! We now increase the timstamp with LCM of schedule because we need to preserve the offsets
        inc = inc.lcm(&schedule);
    }
    return timestamp;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let timestamp = 939;
        let schedules = vec![7, 13, 59, 31, 19];

        let part1 = schedules
            .iter()
            .map(|x| (*x, x - timestamp % x))
            .min_by_key(|x| x.1)
            .unwrap();
        println!("{:?}", part1);
        assert_eq!(part1.0 * part1.1, 295);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("17,x,13,19"), 3417);
        assert_eq!(part2("67,7,59,61"), 754018);
        assert_eq!(part2("67,x,7,59,61"), 779210);
        assert_eq!(part2("1789,37,47,1889"), 1202161486);
    }
}
