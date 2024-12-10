use crate::util::iterators::TruthChecks;
use std::collections::{HashMap, HashSet};

crate::day!("Print Queue" => {
    part_1,
    part_2
});

fn part_1(data: &str) -> usize {
    let (ordering_rules, pages_produced) = parse_input(data);
    pages_produced
        .iter()
        .filter(|page| is_valid_page(*page, &ordering_rules))
        .map(|nums| nums[nums.len() / 2])
        .sum()
}

fn part_2(data: &str) -> usize {
    let (ordering_rules, mut pages_produced) = parse_input(data);
    pages_produced
        .iter_mut()
        .filter(|page| !is_valid_page(*page, &ordering_rules))
        .map(|page| {
            let mut sorted = Vec::new();
            let mut unused: HashSet<usize> = page.iter().copied().collect();
            for _ in 0..page.len() {
                let next_value = unused
                    .iter()
                    .filter(|&&a| {
                        !unused
                            .iter()
                            .map(|&b| {
                                if a == b {
                                    false
                                } else {
                                    is_dependent(b, a, &ordering_rules)
                                }
                            })
                            .any_true()
                    })
                    .copied()
                    .next()
                    .unwrap();
                unused.remove(&next_value);
                sorted.push(next_value);
            }
            return sorted;
        })
        .map(|nums| nums[nums.len() / 2])
        .sum()
}

fn parse_input(input: &str) -> (HashMap<usize, HashSet<usize>>, Vec<Vec<usize>>) {
    let mut lines = input.lines();
    let x_y_pairs: Vec<(usize, usize)> = lines
        .by_ref()
        .take_while(|&l| !l.is_empty())
        .map(|l| {
            let (x, y) = l.split_once("|").unwrap();
            return (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap());
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
        .map(|l| {
            l.split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();
    return (ordering_rules, pages_produced);
}

fn is_valid_page(page: &Vec<usize>, rules: &HashMap<usize, HashSet<usize>>) -> bool {
    let existing: HashSet<usize> = page.iter().copied().collect();
    page.iter()
        .scan(HashSet::new(), |seen, current| {
            let mut valid = true;
            if let Some(pre) = rules.get(current) {
                valid = seen.is_superset(
                    &existing
                        .intersection(pre)
                        .copied()
                        .collect::<HashSet<usize>>(),
                );
            }
            seen.insert(*current);
            Some(valid)
        })
        .all_true()
}

// check if a is dependent on b, based on the ordering rules
fn is_dependent(a: usize, b: usize, rules: &HashMap<usize, HashSet<usize>>) -> bool {
    match rules.get(&a) {
        None => false,
        Some(deps) => deps.contains(&b),
    }
}

crate::test_day!(
"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
",
{
    part_1 => 143,
    part_2 => 123
}
);
