use std::collections::HashMap;

crate::day!("Plutonian Pebbles" => {
    part_1,
    part_2
});

fn part_1(data: &str) -> usize {
    let stones = parse_input(data);
    let mut memo = HashMap::new();
    let result = stones.into_iter().fold(0usize, |sum, stone| sum + update_stone_n_times(stone, 25, &mut memo));
    result
}

fn part_2(data: &str) -> usize {
    let stones = parse_input(data);
    let mut memo = HashMap::new();
    stones.into_iter().fold(0usize, |sum, stone| sum + update_stone_n_times(stone, 75, &mut memo))
}

fn update_stone_n_times(stone: usize, n: usize, prev_results: &mut HashMap<(usize, usize), usize>) -> usize {
    if n == 0 {
        return 1;
    }
    if let Some(res) = prev_results.get(&(stone, n)).copied() {
        return res;
    };
    use StoneResult::*;
    let res = match update_stone(stone) {
        One(new) => update_stone_n_times(new, n - 1, prev_results),
        Two(l, r) => update_stone_n_times(l, n - 1, prev_results) + update_stone_n_times(r, n - 1, prev_results)
    };
    prev_results.insert((stone, n), res);
    res
}

#[derive(Clone, Copy, Debug)]
enum StoneResult {
    One(usize),
    Two(usize, usize)
}

fn update_stone(stone: usize) -> StoneResult {
    use StoneResult::*;
    let digits = stone.checked_ilog10().unwrap_or(0) + 1;
    if stone == 0 {
        One(1)
    } else if digits % 2 == 0 {
        Two(stone / 10usize.pow(digits / 2), stone % 10usize.pow(digits / 2))
    } else {
        One(stone * 2024)
    }
}

fn parse_input(input: &str) -> Vec<usize> {
    input.split(" ").map(|n| n.parse::<usize>().unwrap()).collect()
}

crate::test_day!(
"125 17",
{
    part_1 => 55312,
    part_2 => 65601038650482
}
);