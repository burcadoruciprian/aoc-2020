use counter::Counter;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let raw_input = std::fs::read_to_string("src/input.txt").expect("Error reading the file!");
    let mut ingredients_appearence_counter: Counter<&str, u32> = Counter::new();
    let mut alergen_to_ingredients: HashMap<&str, HashSet<&str>> = HashMap::new();

    raw_input.lines().for_each(|line| {
        let (raw_ingr, raw_alegens) = line.split(" (contains ").collect_tuple().unwrap();
        let ingredients = raw_ingr.split_whitespace().collect::<HashSet<&str>>();
        let alergens = raw_alegens[0..raw_alegens.len() - 1]
            .split(", ")
            .collect::<HashSet<&str>>();

        ingredients_appearence_counter.extend(ingredients.iter().copied());
        alergens.iter().for_each(|a| {
            alergen_to_ingredients
                .entry(a)
                .and_modify(|v| *v = v.intersection(&ingredients).copied().collect())
                .or_insert(ingredients.clone());
        });
    });

    //dbg!(ingredients_appearence_counter.clone());
    dbg!(alergen_to_ingredients.clone());

    let inert_ingredients = alergen_to_ingredients.values().fold(
        ingredients_appearence_counter
            .keys()
            .copied()
            .collect::<HashSet<&str>>(),
        |acc, v| acc.difference(&v).copied().collect(),
    );
    //dbg!(inert_ingredients.clone());

    println!(
        "Part1: {}",
        inert_ingredients
            .iter()
            .fold(0, |acc, i| acc + ingredients_appearence_counter[i])
    );

    loop {
        if alergen_to_ingredients.values().all(|v| v.len() == 1) {
            break;
        }

        let ingredients = alergen_to_ingredients
            .iter()
            .filter_map(|(k, v)| match v.len() == 1 {
                true => Some(v.iter().last().unwrap()),
                false => None,
            })
            .copied()
            .collect::<HashSet<&str>>();

        alergen_to_ingredients.iter_mut().for_each(|(k, v)| {
            if v.len() == 1 {
                return;
            }
            *v = v.difference(&ingredients).copied().collect();
        });
    }

    dbg!(alergen_to_ingredients.clone());

    let sorted_ingredients = alergen_to_ingredients
        .clone()
        .keys()
        .sorted()
        .map(|v| *alergen_to_ingredients[v].iter().next().unwrap())
        .collect::<Vec<&str>>();

    dbg!(sorted_ingredients.clone());
    println!("Part2: {}", sorted_ingredients.join(","));
}
