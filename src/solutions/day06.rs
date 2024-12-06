use crate::util::grid_positions::{Direction, Position};
use std::collections::{HashMap, HashSet};

crate::day!("Guard Gallivant" => {
    part_1,
    part_2
});

type Grid = HashMap<Position, Token>;

#[derive(Clone, Copy, Debug)]
enum Token {
    Empty,
    Obstacle,
    Guard,
}

fn part_1(data: &str) -> usize {
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

fn part_2(data: &str) -> usize {
    let map = parse_input(data);
    let mut pos = get_initial_guard_position(&map);
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
        positions.insert(pos);
    }
    let mut new_obstacles: HashSet<Position> = HashSet::new();
    let guard_position = get_initial_guard_position(&map);
    for pos in positions {
        if pos == guard_position {
            continue;
        }
        if can_loop(pos, &map) {
            new_obstacles.insert(pos + dir);
        }
    }
    new_obstacles.len()
}

/// check whether inserting an obstacle at the given position creates a loop
fn can_loop(pos: Position, map: &Grid) -> bool {
    let mut test_map = map.clone();
    test_map.insert(pos, Token::Obstacle);
    let mut pos = get_initial_guard_position(map);
    let mut dir = Direction::up();
    let mut visited: HashSet<Position> = HashSet::new();
    let mut consecutive_repeats = 0;
    visited.insert(pos);
    loop {
        match test_map.get(&(pos + dir)) {
            Some(Token::Obstacle) => {
                dir = dir.rotate_right();
            }
            None => {
                return false;
            }
            _ => {
                pos = pos + dir;
                if !visited.insert(pos) {
                    consecutive_repeats += 1;
                    if consecutive_repeats == visited.len() {
                        return true;
                    }
                } else {
                    consecutive_repeats = 0;
                }
            }
        }
    }
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
