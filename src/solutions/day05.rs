
use std::collections::{HashMap, HashSet};

use crate::input;

fn parse_input(input: &str) -> (HashMap<usize, HashSet<usize>>, Vec<Vec<usize>>) {
    let mut lines = input.lines();
    let x_y_pairs: Vec<(usize, usize)> = lines
        .by_ref()
        .take_while(|&l| !l.is_empty())
        .map(|l| {
            let (x, y) = l.split_once("|").unwrap();
            return (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        })
        .collect();
    let mut ordering_rules: HashMap<usize, HashSet<usize>> = HashMap::new();
    for (x, y) in x_y_pairs {
        if let Some(set) = ordering_rules.get_mut(&y) {
            set.insert(x);
        } else {
            let mut set = HashSet::new();
            set.insert(x);
            ordering_rules.insert(y, set);
        }
    }
    let pages_produced: Vec<Vec<usize>> = lines
        .map(|l| l.split(",").map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>())
        .collect();
    return (ordering_rules, pages_produced)
}

fn part_1(data: &str) {
    let (ordering_rules, pages_produced) = parse_input(data);
    let valid: usize = pages_produced.iter().filter(|page| {
        let existing: HashSet<usize> = page.iter().copied().collect();
        page.iter().scan(HashSet::new(), |seen, current| {
            let mut valid = true;
            if let Some(pre) = ordering_rules.get(current) {
                valid = seen
                    .is_superset(&existing.intersection(pre).copied().collect::<HashSet<usize>>());
            }
            seen.insert(*current);
            Some(valid)
        }).all(|b| b)
    })
    .map(|nums| {
        nums[nums.len() / 2]
    }).sum();
    println!("Valid: {valid}");
}

fn part_2(data: &str) {
    
}

pub fn main(input_dir: &str) {
    println!("--- Day 5: ---");
    let data = input::load(input_dir, 5, None);
    println!("Part 1:");
    part_1(&data);
    println!("Part 2:");
    part_2(&data);
}
