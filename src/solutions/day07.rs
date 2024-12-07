use indicatif::ProgressBar;
use itertools::Itertools;
use itertools::repeat_n;

crate::day!("Bridge Repair" + bars => {
    part_1,
    part_2
});

fn part_1(data: &str, _: &ProgressBar) -> usize {
    let data = parse_input(data);
    data.iter().filter(|(test_value, values)| {
        repeat_n(vec![Operator::Mul,Operator::Add].iter(), values.len() - 1)
            .multi_cartesian_product()
            .map(|ops| apply_operators(values, ops))
            .any(|v| v == *test_value)
    }).fold(0, |acc, (test_value, _)| acc + *test_value)
}

fn part_2(data: &str, bar: &ProgressBar) -> usize {
    let data = parse_input(data);
    bar.set_length(data.len() as u64);
    data.iter().filter(|(test_value, values)| {
        bar.inc(1);
        repeat_n(vec![Operator::Mul,Operator::Add,Operator::Con].iter(), values.len() - 1)
            .multi_cartesian_product()
            .map(|ops| apply_operators(values, ops))
            .any(|v| v == *test_value)
    }).fold(0, |acc, (test_value, _)| acc + *test_value)
}

#[derive(Clone, Copy, Debug)]
enum Operator {
    Mul,
    Add,
    Con
}

fn apply_operators(values: &Vec<usize>, ops: Vec<&Operator>) -> usize {
    let mut op_iter = ops.iter().copied();
    let mut total = values[0];
    for &v in values[1..].iter() {
        total = apply_operator(total, v, *op_iter.next().unwrap())
    }
    total
}

fn apply_operator(lhs: usize, rhs: usize, op: Operator) -> usize {
    match op {
        Operator::Add => lhs + rhs,
        Operator::Mul => lhs * rhs,
        Operator::Con => format!("{lhs}{rhs}").parse::<usize>().unwrap()
    }
}

fn parse_input(input: &str) -> Vec<(usize, Vec<usize>)> {
    input.lines()
        .map(|l| {
            let (first, rest) = l.split_once(": ").unwrap();
            let rest_vec = rest.split(" ").map(|d| d.parse::<usize>().unwrap()).collect();
            (first.parse::<usize>().unwrap(), rest_vec)
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn p1_test() {
        assert_eq!(part_1(TEST, &ProgressBar::hidden()), 3749);
    }

    #[test]
    fn p2_test() {
        assert_eq!(part_2(TEST, &ProgressBar::hidden()), 11387);
    }
}