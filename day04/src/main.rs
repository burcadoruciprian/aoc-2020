use regex::Regex;
use std::collections::HashMap;

fn is_valid_passport_1(passport: &HashMap<&str, &str>) -> bool {
    return vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .all(|x| passport.contains_key(x));
}

fn is_valid_passport_2(passport: &HashMap<&str, &str>) -> bool {
    if !is_valid_passport_1(passport) {
        return false;
    }

    let byr: i32 = passport["byr"].parse().unwrap_or_default();
    if byr < 1920 || byr > 2002 {
        return false;
    }

    let iyr: i32 = passport["iyr"].parse().unwrap_or_default();
    if iyr < 2010 || iyr > 2020 {
        return false;
    }

    let eyr: i32 = passport["eyr"].parse().unwrap_or_default();
    if eyr < 2020 || eyr > 2030 {
        return false;
    }

    let re = Regex::new(r"(\d+)(cm|in)").unwrap();
    let result = re.captures(passport["hgt"]);
    if result.is_none() {
        return false;
    }
    let result = result.unwrap();

    let htg_no: i32 = result.get(1).unwrap().as_str().parse().unwrap_or_default();
    let hgt_um: &str = result.get(2).unwrap().as_str();
    if ((hgt_um == "cm") && (htg_no < 150 || htg_no > 193))
        || ((hgt_um == "in") && (htg_no < 59 || htg_no > 76))
    {
        return false;
    }

    let hcl = passport["hcl"];
    if hcl.len() != 7
        || !hcl.chars().enumerate().all(|(i, c)| {
            if i == 0 && c == '#' {
                return true;
            }
            return match c {
                '0'..='9' => true,
                'a'..='f' => true,
                _ => false,
            };
        })
    {
        return false;
    }

    let ecl = passport["ecl"];
    if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl) {
        return false;
    }

    let pid = passport["pid"];
    if pid.len() != 9 || !pid.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    return true;
}

fn main() {
    let raw_input = std::fs::read_to_string("src/input.txt").expect("Error reading the file!");
    let mut passports: Vec<HashMap<&str, &str>> = Vec::new();
    passports.push(HashMap::new());
    for line in raw_input.lines() {
        if line.is_empty() {
            passports.push(HashMap::new());
        }
        let tmp: Vec<&str> = line.split(&[' ', ':'][..]).collect();
        for w in tmp.chunks_exact(2) {
            passports.last_mut().unwrap().insert(w[0], w[1]);
        }
    }

    println!(
        "Part1: {}",
        passports.iter().fold(0, |acc, x| {
            if is_valid_passport_1(x) {
                acc + 1
            } else {
                acc
            }
        })
    );
    println!(
        "Part2: {}",
        passports.iter().fold(0, |acc, x| {
            if is_valid_passport_2(x) {
                acc + 1
            } else {
                acc
            }
        })
    );
}
