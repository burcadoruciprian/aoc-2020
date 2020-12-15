use std::collections::HashMap;
fn main() {
    let input: Vec<usize> = vec![2, 0, 1, 9, 5, 19];
    println!("Part1: {}", speak(input.clone(), 2020));
    println!("Part2: {}", speak(input, 30000000));
}

fn speak(input: Vec<usize>, turns: usize) -> usize {
    let mut memory = HashMap::new();
    input.iter().enumerate().for_each(|(i, v)| {
        memory.insert(*v, i + 1);
    });

    let mut turn = input.len();
    let mut speak = *input.iter().last().unwrap();
    //dbg!(turn);
    //dbg!(speak);
    loop {
        if turn == turns {
            break;
        }
        let last = memory.entry(speak).or_insert(turn); //Use muttable reference to speedup the hash update
        if *last == turn {
            speak = 0;
        } else {
            speak = turn - *last;
            *last = turn; //Update the entry in hash
        }
        turn += 1;
        //dbg!(turn);
        //dbg!(speak);
    }

    return *memory.entry(speak).key();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut input: Vec<usize> = vec![0, 3, 6];
        assert_eq!(speak(input, 2020), 436);

        input = vec![3,1,2];
        assert_eq!(speak(input, 2020), 1836);
    }
}
