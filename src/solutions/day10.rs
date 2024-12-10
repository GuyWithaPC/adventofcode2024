use std::collections::{HashMap, HashSet};

use indicatif::ProgressBar;
use itertools::Itertools;

use crate::util::positions::Vec2;

crate::day!("Hoof It" + bars => {
    part_1,
    part_2
});

type Position = Vec2<isize>;
type Grid = HashMap<Position, usize>;

fn part_1(data: &str, _: &ProgressBar) -> usize {
    let map = parse_input(data);
    let trailheads = get_number(&map, 0);
    trailheads.iter().map(|&start| {
        get_number(&filter_unreachable(&map, start), 9).len()
    }).sum()
}

fn part_2(data: &str, bar: &ProgressBar) -> usize {
    let map = parse_input(data);
    let starts = get_number(&map, 0);
    let ends = get_number(&map, 9);
    bar.set_length((starts.len() * ends.len()) as u64);
    starts.iter().cartesian_product(ends).map(|(&start, end)| {
        bar.inc(1);
        distinct_trails(&map, start, end)
    }).sum()
}

fn parse_input(input: &str) -> Grid {
    input.lines().enumerate().flat_map(|(y, row)| {
        row.chars().enumerate().map(move |(x, c)| {
            (Position::new(x as isize, y as isize), c.to_digit(10).unwrap() as usize)
        })
    }).collect()
}

fn get_number(map: &Grid, num: usize) -> Vec<Position> {
    map.iter().filter_map(|(&pos, &value)| {
        if value == num {
            Some(pos)
        } else {
            None
        }
    }).collect()
}

/// filter out unreachable spots in a copy of the given map
fn filter_unreachable(map: &Grid, start: Position) -> Grid {
    let directions: [Vec2<isize>;4] = [Vec2::up(), Vec2::right(), Vec2::down(), Vec2::left()];
    let mut filtered: HashMap<Position, usize> = HashMap::new();
    let mut to_visit: Vec<(usize, Position)> = Vec::new();
    filtered.insert(start, map.get(&start).copied().unwrap());
    for dir in directions {
        to_visit.push((0, start + dir));
    }
    loop {
        let check = to_visit.pop();
        if check.is_none() {
            break;
        }
        let (last, check) = check.unwrap();
        if let Some(check_val) = map.get(&check).copied() {
            if check_val != 0 && check_val - 1 == last {
                filtered.insert(check, check_val);
                for dir in directions {
                    if filtered.contains_key(&(check + dir)) {
                        continue;
                    }
                    to_visit.push((check_val, check + dir));
                }
            }
        }
    }
    return filtered;
}

fn distinct_trails(map: &Grid, start: Position, end: Position) -> usize {
    let directions: [Vec2<isize>;4] = [Vec2::up(), Vec2::right(), Vec2::down(), Vec2::left()];
    let mut to_visit: Vec<(usize, Position)> = Vec::new();
    let mut paths = 0;
    to_visit.push((0, start));
    loop {
        let check = to_visit.pop();
        if check.is_none() {
            break;
        }
        let (check_val, check) = check.unwrap();
        if check == end {
            paths += 1;
            continue;
        }
        for dir in directions {
            if let Some(v) = map.get(&(check + dir)).copied() {
                if v == check_val + 1 {
                    to_visit.push((v, check + dir));
                }
            }
        }
    }
    return paths;
}

crate::test_day!(
"
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
" + bars,
{
    part_1 => 36,
    part_2 => 81
}
);