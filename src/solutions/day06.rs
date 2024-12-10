use indicatif::ProgressBar;

use crate::util::positions::Vec2;
use std::collections::{HashMap, HashSet};

crate::day!("Guard Gallivant" + bars => {
    part_1,
    part_2
});

type Position = Vec2<isize>;
type Direction = Vec2<isize>;
type Grid = HashMap<Position, Token>;

#[derive(Clone, Copy, Debug)]
enum Token {
    Empty,
    Obstacle,
    Guard,
}

fn part_1(data: &str, _: &ProgressBar) -> usize {
    let map = parse_input(data);
    let mut pos = get_initial_guard_position(&map);
    let mut dir = Direction::up();
    let mut positions: HashSet<Position> = HashSet::new();
    positions.insert(pos);
    'outer: loop {
        match map.get(&(pos + dir)) {
            Some(Token::Obstacle) => dir = dir.rotate_right(),
            None => break 'outer,
            _ => {
                pos = pos + dir;
                positions.insert(pos);
            }
        }
    }
    positions.len()
}

fn part_2(data: &str, bar: &ProgressBar) -> usize {
    bar.set_length(part_1(data, bar) as u64);
    let mut map = parse_input(data);
    let mut new_obstacles: HashSet<Position> = HashSet::new();
    let guard_position = get_initial_guard_position(&map);
    let mut pos = guard_position;
    let mut dir = Direction::up();
    let mut positions: HashSet<Position> = HashSet::new();
    positions.insert(pos);
    'outer: loop {
        match map.get(&(pos + dir)) {
            Some(Token::Obstacle) => {
                dir = dir.rotate_right();
            }
            None => break 'outer,
            _ => {
                pos = pos + dir;
            }
        }
        if can_loop(pos, dir, &positions, &mut map) {
            new_obstacles.insert(pos + dir);
        }
        positions.insert(pos);
        bar.inc(1);
    }
    new_obstacles.len()
}

/// check whether inserting an obstacle in front of the given position creates a loop
fn can_loop(
    mut pos: Position,
    mut dir: Direction,
    already_seen: &HashSet<Position>,
    map: &mut Grid,
) -> bool {
    match map.get(&(pos + dir)) {
        Some(Token::Empty) => {}
        _ => {
            return false;
        }
    };
    let test_pos = pos + dir;
    map.insert(test_pos, Token::Obstacle);
    if already_seen.contains(&(pos + dir)) {
        pos = get_initial_guard_position(map);
        dir = Direction::up();
    }
    let mut visited: HashSet<(Position, Direction)> = HashSet::new();
    let mut is_loop = false;
    'outer: loop {
        match map.get(&(pos + dir)) {
            Some(Token::Obstacle) => {
                dir = dir.rotate_right();
            }
            None => {
                break 'outer;
            }
            _ => {
                pos = pos + dir;
                if !visited.insert((pos, dir)) {
                    is_loop = true;
                    break 'outer;
                }
            }
        }
    }
    map.insert(test_pos, Token::Empty);
    return is_loop;
}

fn get_initial_guard_position(map: &Grid) -> Position {
    map.iter()
        .filter_map(|(pos, token)| match token {
            Token::Guard => Some(*pos),
            _ => None,
        })
        .next()
        .unwrap()
}

fn parse_input(input: &str) -> Grid {
    input
        .lines()
        .enumerate()
        .map(|(row, l)| {
            l.chars().enumerate().map(move |(col, c)| match c {
                '#' => (Position::new(col as isize, row as isize), Token::Obstacle),
                '^' => (Position::new(col as isize, row as isize), Token::Guard), // guard facing up
                _ => (Position::new(col as isize, row as isize), Token::Empty),
            })
        })
        .flatten()
        .collect()
}

crate::test_day!(
"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
" + bars,
{
    part_1 => 41,
    part_2 => 6
}
);
